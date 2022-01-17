use crate::helpers::{spawn_app, TestUser, UserField};

#[tokio::test]
async fn valid_user_add_send_200() {
    let test_app = spawn_app().await;
    let test_user = TestUser::valid_user();
    let resp = test_user.add_user(&test_app.address).await.unwrap();

    assert_eq!(200, resp.status().as_u16());

    let saved_user = sqlx::query!("SELECT username, email FROM users")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved_user.username, test_user.username);
    assert_eq!(saved_user.email, test_user.email);
}

#[tokio::test]
async fn missing_field_user_add_send_400() {
    let test_app = spawn_app().await;
    let tests = [
        (
            TestUser::missing_field(UserField::Username),
            "missing username",
        ),
        (TestUser::missing_field(UserField::Email), "missing email"),
        (
            TestUser::missing_field(UserField::Password),
            "missing password",
        ),
        (
            TestUser::missing_field(UserField::PasswordVer),
            "missing password_ver",
        ),
    ];

    for (b, e) in tests {
        let resp = b.add_user(&test_app.address).await.unwrap();
        assert_eq!(400, resp.status().as_u16(), "failed to fail with {}", e);
    }
}

#[tokio::test]
async fn fields_present_invalid_submissions() {
    let test_app = spawn_app().await;
    let tests = [
        (
            TestUser::invalid_field(UserField::Username),
            "invalid username",
        ),
        (TestUser::invalid_field(UserField::Email), "invalid email"),
        (
            TestUser::invalid_field(UserField::Password),
            "invalid password",
        ),
        (
            TestUser::invalid_field(UserField::PasswordVer),
            "mismatching password_ver",
        ),
    ];

    for (b, e) in tests {
        let resp = b.add_user(&test_app.address).await.unwrap();
        assert_eq!(400, resp.status().as_u16(), "failed to fail with {}", e);
    }
}
