use crate::routes::{health_check, index};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;
use tera::Tera;
use tracing_actix_web::TracingLogger;

pub struct AppData {
    pub template: Arc<Tera>,
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let tera = Arc::new(match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("{} error parsing templates exiting program", e);
            std::process::exit(1);
        }
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData {
                template: tera.clone(),
            }))
            .wrap(TracingLogger::default())
            .service(index)
            .service(health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
