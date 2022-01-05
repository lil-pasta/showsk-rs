use crate::helpers::spawn_app;
use reqwest::Client;

#[tokio::test]
async fn health_check() {
    let test_app = spawn_app().await;
    let client = Client::new();
    let endpoint = format!("{}/health_check", test_app.address);

    let response = client
        .get(&endpoint)
        .send()
        .await
        .expect("failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
