use crate::helpers::{spawn_app, TestUser};

#[tokio::test]
async fn valid_login_redirects_to_admin_dashboard() {
    let test_app = spawn_app().await;
    let test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();
    let client = TestUser::test_client();
    // make sure user was added to db
    assert_eq!(200, add_user.status().as_u16());

    let login_user = test_user
        .login_user(&test_app.address, &client)
        .await
        .unwrap();

    assert_eq!(&303, &login_user.status().as_u16());
    assert_eq!(
        "/admin/dashboard",
        login_user.headers().get("Location").unwrap()
    );

    let admin_dashboard = test_user
        .get_admin_dashboard(&test_app.address, &client)
        .await
        .unwrap();
    let dashboard_html = admin_dashboard.text().await.unwrap();
    assert!(dashboard_html.contains(&format!("Welcome {}", test_user.username)));
}

#[tokio::test]
async fn missing_username_returns_303() {
    let test_app = spawn_app().await;
    let mut test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();
    let client = TestUser::test_client();

    assert_eq!(200, add_user.status().as_u16());

    test_user.username = "";
    let login_user = test_user
        .login_user(&test_app.address, &client)
        .await
        .unwrap();

    // get html response from login form
    let resp = client
        .get(&format!("{}/login", &test_app.address))
        .send()
        .await
        .expect("could not get login form");

    let login_html = resp.text().await.unwrap();
    assert_eq!(303, login_user.status().as_u16());
    // see if the resulting html contains the invalid login message!
    assert!(login_html.contains(r#"<p>Username and password are incorrect</p>"#));

    let resp = client
        .get(&format!("{}/login", &test_app.address))
        .send()
        .await
        .expect("could not get login form");
    let login_html = resp.text().await.unwrap();

    // see if the resulting html contains the invalid login message!
    // it should not after the second call since it is not on the
    // heels of a failed login attempt
    assert!(!login_html.contains(r#"<p>Username and password are incorrect</p>"#));
}

#[tokio::test]
async fn missing_password_returns_303() {
    let test_app = spawn_app().await;
    let mut test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();
    let client = TestUser::test_client();

    assert_eq!(200, add_user.status().as_u16());

    test_user.password = "";
    let login_user = test_user
        .login_user(&test_app.address, &client)
        .await
        .unwrap();

    assert_eq!(303, login_user.status().as_u16());
}

#[tokio::test]
async fn incorrect_password_valid_username_returns_303() {
    let test_app = spawn_app().await;
    let mut test_user = TestUser::valid_user();
    let add_user = test_user.add_user(&test_app.address).await.unwrap();
    let client = TestUser::test_client();

    assert_eq!(200, add_user.status().as_u16());

    test_user.password = "wrong_password";

    let login_user = test_user
        .login_user(&test_app.address, &client)
        .await
        .unwrap();

    assert_eq!(303, login_user.status().as_u16());
}
