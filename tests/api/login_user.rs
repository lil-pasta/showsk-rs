use crate::helpers::spawn_app;
use reqwest::Client;

#[tokio::test]
async fn valid_login_returns_200() {}

#[tokio::test]
async fn missing_username_returns_401() {}

#[tokio::test]
async fn missing_password_returns_401() {}

#[tokio::test]
async fn incorrect_password_valid_username_returns_401() {}
