use once_cell::sync::Lazy;
use showsk_rs::startup::run;
use showsk_rs::telemetry::{init_subscriber, subscriber_set_up};

pub struct TestApp {
    pub address: String,
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

    // start a server bound to that random port
    let server = run(listener).expect("failed to start test server");
    let _ = tokio::spawn(server);

    TestApp { address: addr }
}

pub async fn spawn_test_db() -> PgPool {
    let db_name = uuid::Uuid
