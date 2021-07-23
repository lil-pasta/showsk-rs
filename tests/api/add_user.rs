use crate::helpers::spawn_app;
use reqwest::Client;

#[actix_rt::test]
async fn valid_user_add_send_200() {
    let test_app = spawn_app().await;
    let client = Client::new();
    let body = "username=lilpasta&email=pasta%40pasta.com&password=he!he11o&password_ver=he!he11o";

    let resp = client
        .post(&format!("{}/add_user", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to reach route add_user");

    assert_eq!(200, resp.status().as_u16());

    let saved_user = sqlx::query!("SELECT username, email FROM users")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved_user.username, "lilpasta");
    assert_eq!(saved_user.email, "pasta@pasta.com");
}

#[actix_rt::test]
async fn missing_field_user_add_send_400() {
    let test_app = spawn_app().await;
    let client = Client::new();
    let bodies = [
        (
            "email=pasta%40pasta.com&password=he!he11o&password_ver=he!he11o",
            "missing username",
        ),
        (
            "username=lilpasta&password=he!he11o&password_ver=he!he11o",
            "missing email",
        ),
        (
            "username=lilpasta&email=pasta%40pasta.com&password_ver=he!he11o",
            "missing password",
        ),
        (
            "username=lilpasta&email=pasta%40pasta.com&password=he!he11o",
            "missing password_ver",
        ),
    ];

    for (b, e) in bodies {
        let resp = client
            .post(&format!("{}/add_user", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(b)
            .send()
            .await
            .expect("failed to reach route add_user");
        assert_eq!(400, resp.status().as_u16(), "failed to fail with {}", e);
    }
}

#[actix_rt::test]
async fn fields_present_invalid_submissions() {
    let test_app = spawn_app().await;
    let client = Client::new();
    let bodies = [
        (
            "username=fn%20malware()%20->%20mal&email=pasta%40pasta.com&password=he!he11o&password_ver=he!he11o",
            "invalid username",
        ),
        (
            "username=lilpasta&email=pasta.com&password=he!he11o&password_ver=he!he11o",
            "invalid email",
        ),
        (
            "username=lilpasta&email=pasta%40pasta.com&password=he11o&password_ver=he11o",
            "invalid password",
        ),
        (
            "username=lilpasta&email=pasta%40pasta.com&password=he!he11o&password_ver=he!heoo11o",
            "mismatching password_ver",
        ),
    ];

    for (b, e) in bodies {
        let resp = client
            .post(&format!("{}/add_user", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(b)
            .send()
            .await
            .expect("failed to reach route add_user");
        assert_eq!(400, resp.status().as_u16(), "failed to fail with {}", e);
    }
}
