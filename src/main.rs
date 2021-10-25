use showsk_rs::configuration::get_conf;
use showsk_rs::startup::run;
use showsk_rs::telemetry::{init_subscriber, subscriber_set_up};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get application config
    let configuration = get_conf().expect("failed to load configurations");
    let address = configuration.application_address();
    let listener = TcpListener::bind(address)?;
    let up_path = configuration.application.upload_path;

    // set up database
    let db_pool = PgPoolOptions::new()
        .connect_with(configuration.database.connection_with_db())
        .await
        .expect("failed to connect to db pool");

    // set up telemetry
    let subscriber = subscriber_set_up("waterboy".into(), "info".into());
    init_subscriber(subscriber);

    let uppath = std::env::current_dir().unwrap().join(&up_path);
    if !std::path::Path::new(&uppath).is_dir() {
        std::fs::create_dir_all(&uppath.to_str().unwrap())?;
    }
    run(listener, db_pool, up_path)?.await
}
