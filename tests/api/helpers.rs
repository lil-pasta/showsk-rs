use once_cell::sync::Lazy;
use showsk_rs::configuration::{get_conf, DatabaseSetting};
use showsk_rs::startup::Application;
use showsk_rs::telemetry::{init_subscriber, subscriber_set_up};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub up_path: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = subscriber_set_up("waterboy_test".into(), "info".into());
    init_subscriber(subscriber);
});

pub async fn spawn_app() -> TestApp {
    // tracing for test app
    Lazy::force(&TRACING);

    // spawn your test db with a random db name
    let conf = {
        let mut c = get_conf().expect("failed to bind path");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    let db_pool = spawn_test_db(&conf.database).await;
    let upp = conf.application.upload_path.clone();
    let application = Application::build(conf)
        .await
        .expect("failed to start test server");
    let port = application.get_port();
    let addr = format!("http://127.0.0.1:{}", &port);
    // start a server bound to that random port
    let _ = tokio::spawn(application.run_until_stop());

    TestApp {
        address: addr,
        up_path: upp,
        db_pool,
    }
}

pub async fn spawn_test_db(database: &DatabaseSetting) -> PgPool {
    let mut db_connection = PgConnection::connect_with(&database.connection_without_db())
        .await
        .expect("could not connect to postgres");

    db_connection
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            database.database_name
        ))
        .await
        .expect("failed to create test db");

    let pool = PgPool::connect_with(database.connection_with_db())
        .await
        .expect("could not connect to test db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");
    pool
}
