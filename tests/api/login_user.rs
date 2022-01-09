use tracing::instrument;

use crate::helpers::{spawn_app, TestUser, UserField};

#[tokio::test]
async fn valid_login_returns_200() {
    let test_app = spawn_app().await;
    let test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();

    assert_eq!(200, add_user.status().as_u16());

    let saved_user = sqlx::query!("SELECT user_id FROM users")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    let login_user = test_user.login_user(&test_app.address).await.unwrap();

    assert_eq!(200, login_user.status().as_u16());
    assert_eq!(
        saved_user.user_id.to_string(),
        login_user.text().await.unwrap()
    );
}

#[tokio::test]
#[tracing::instrument(name = "missing user name returns 401")]
async fn missing_username_returns_401() {
    let test_app = spawn_app().await;
    let mut test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();

    assert_eq!(200, add_user.status().as_u16());

    test_user.username = "";
    let login_user = test_user.login_user(&test_app.address).await.unwrap();

    assert_eq!(401, login_user.status().as_u16());
}

#[tokio::test]
async fn missing_password_returns_401() {
    let test_app = spawn_app().await;
    let mut test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();

    assert_eq!(200, add_user.status().as_u16());

    test_user.password = "";
    let login_user = test_user.login_user(&test_app.address).await.unwrap();

    assert_eq!(401, login_user.status().as_u16());
}

#[tokio::test]
async fn incorrect_password_valid_username_returns_401() {
    let test_app = spawn_app().await;
    let mut test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();

    assert_eq!(200, add_user.status().as_u16());

    test_user.password = "wrong_password";

    let login_user = test_user.login_user(&test_app.address).await.unwrap();

    assert_eq!(401, login_user.status().as_u16());
}
