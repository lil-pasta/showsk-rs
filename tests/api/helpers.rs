use once_cell::sync::Lazy;
use showsk_rs::configuration::{get_conf, DatabaseSetting};
use showsk_rs::startup::run;
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

    // create a listener on a random port and save the new app address
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("failed to set up test listener");
    let port = listener.local_addr().unwrap().port();
    let addr = format!("http://127.0.0.1:{}", port);

    // spawn your test db with a random db name
    let mut conf = get_conf().expect("failed to bind path");
    conf.database.database_name = Uuid::new_v4().to_string();
    let db_pool = spawn_test_db(&conf.database).await;
    let upp = conf.application.upload_path;

    // start a server bound to that random port
    let server = run(listener, db_pool.clone(), upp.clone()).expect("failed to start test server");
    let _ = tokio::spawn(server);

    TestApp {
        address: addr,
        up_path: upp,
        db_pool: db_pool,
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
