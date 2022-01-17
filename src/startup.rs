use crate::configuration;
use crate::routes::{
    add_user, admin_dashboard, health_check, index, login_form, login_validate, submit_post,
    write_post,
};
use actix_files::Files;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;
use tera::Tera;
use tracing_actix_web::TracingLogger;

#[derive(Debug)]
pub struct AppData {
    pub template: Arc<Tera>,
    pub db_pool: web::Data<PgPool>,
    pub upload_path: String,
    pub hmac_secret: Secret<String>,
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(conf: configuration::Settings) -> Result<Self, anyhow::Error> {
        let address = conf.application_address();
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let secret = conf.application.hmac_secret;
        let up_path = conf.application.upload_path;
        let redis_uri = conf.application.redis_uri;

        // set up database
        let db_pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_with(conf.database.connection_with_db())
            .await?;

        let server = run(listener, db_pool, up_path, secret, redis_uri).await?;
        Ok(Self { port, server })
    }

    pub async fn run_until_stop(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    up_path: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let tera = Arc::new(match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("{} error parsing templates exiting program", e);
            std::process::exit(1);
        }
    });
    let db = web::Data::new(db_pool);
    let key = Key::from(hmac_secret.expose_secret().as_bytes());
    let cookie_message_store = CookieMessageStore::builder(key.clone()).build();
    let flash_message_framework = FlashMessagesFramework::builder(cookie_message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData {
                template: tera.clone(),
                db_pool: db.clone(),
                upload_path: up_path.clone(),
                hmac_secret: hmac_secret.clone(),
            }))
            .wrap(TracingLogger::default())
            .wrap(flash_message_framework.clone())
            .wrap(SessionMiddleware::new(redis_store.clone(), key.clone()))
            .service(
                Files::new("/static/css", "static/css/")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
            .service(Files::new("/static/uploads", "static/uploads/"))
            .service(index)
            .service(health_check)
            .service(add_user)
            .service(submit_post)
            .service(write_post)
            .service(login_form)
            .service(login_validate)
            .service(admin_dashboard)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
