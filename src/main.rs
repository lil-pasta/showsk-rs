use showsk_rs::configuration::get_conf;
use showsk_rs::startup::Application;
use showsk_rs::telemetry::{init_subscriber, subscriber_set_up};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // get application config
    let configuration = get_conf().expect("failed to load configurations");
    let application = Application::build(configuration).await?;

    // set up telemetry
    let subscriber = subscriber_set_up("showsk-rs".into(), "info".into());
    init_subscriber(subscriber);

    application.run_until_stop().await?;
    Ok(())
}
