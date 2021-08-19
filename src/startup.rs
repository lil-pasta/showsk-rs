use crate::routes::{add_user, health_check, index, submit_post, write_post};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
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
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    up_path: String,
) -> Result<Server, std::io::Error> {
    let tera = Arc::new(match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("{} error parsing templates exiting program", e);
            std::process::exit(1);
        }
    });
    let db = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData {
                template: tera.clone(),
                db_pool: db.clone(),
                upload_path: up_path.clone(),
            }))
            .wrap(TracingLogger::default())
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
    })
    .listen(listener)?
    .run();

    Ok(server)
}
