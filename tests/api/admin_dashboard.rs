use crate::helpers::{spawn_app, TestUser};

#[tokio::test]
async fn only_logged_in_users_can_view_admin_dashboard() {
    let app = spawn_app().await;
    let test_user = TestUser::valid_user();
    let client = TestUser::test_client();

    let admin_res = test_user
        .get_admin_dashboard(&app.address, &client)
        .await
        .unwrap();

    assert_eq!(&303, &admin_res.status().as_u16());
    assert_eq!("/login", admin_res.headers().get("Location").unwrap());
}
