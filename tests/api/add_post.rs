use crate::helpers::spawn_app;
use reqwest;

#[tokio::test]
async fn valid_post_add_send_200() {
    // this is big enough to be a multipart lol...
    let text_body: String = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer in elit libero. Nunc cursus, dui et ornare vehicula, lacus metus ullamcorper lacus, vel scelerisque orci massa eu erat. Aenean pulvinar velit sem, et interdum turpis mattis non. Suspendisse quam eros, commodo ac quam a, condimentum hendrerit ante. Donec laoreet malesuada lacinia. Ut nec massa sed leo vulputate varius. Maecenas eu justo eget turpis lobortis fringilla sed vitae ante. Aliquam mattis quis elit ac efficitur.

Phasellus dolor arcu, scelerisque sed erat ultrices, pellentesque auctor nisi. Sed posuere ante ac dapibus elementum. Fusce bibendum purus nisi, quis interdum sapien facilisis sed. Suspendisse venenatis aliquet aliquam. Etiam sed risus nibh. Aliquam quis placerat tortor. Morbi varius in neque eget venenatis. Donec scelerisque eu neque id ornare. Vivamus elementum aliquam lectus, ac rutrum libero tristique sed. Duis mi purus, vestibulum semper arcu dapibus, faucibus tincidunt nulla.

Sed vel nisi auctor, aliquam libero eu, vehicula arcu. Quisque sed quam turpis. Etiam efficitur mi a molestie imperdiet. Nullam elementum ullamcorper odio, in scelerisque dolor ullamcorper vel. Sed ornare arcu porta felis condimentum, a sagittis nibh tristique. Etiam rhoncus at sapien in laoreet. Nullam et leo purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae;

Interdum et malesuada fames ac ante ipsum primis in faucibus. Nullam euismod porttitor auctor. Curabitur eleifend, ex eu ultricies elementum, sapien justo dapibus lectus, et auctor ligula leo eleifend lectus. Aliquam aliquam dignissim iaculis. Praesent dictum felis neque, nec ultrices neque faucibus at. Nulla sit amet tortor sed nunc feugiat mollis ut eleifend nulla. Sed eu commodo neque, ut sagittis velit. Ut laoreet purus eget justo condimentum sagittis at nec elit. Proin condimentum ipsum vitae libero cursus, consectetur suscipit mauris facilisis. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Vivamus lacinia nisi sed tellus dignissim, ut faucibus urna fermentum. In hac habitasse platea dictumst. Mauris dapibus tincidunt lorem sit amet sollicitudin.

Pellentesque vitae cursus nisi. Mauris a tempor odio. Morbi magna libero, fringilla a odio ut, ullamcorper ultricies ante. Aliquam ornare neque odio. Integer ac volutpat purus, ac accumsan sem. Quisque non suscipit metus, viverra viverra tellus. Nulla odio urna, bibendum at accumsan a, condimentum a tellus."#.to_string();
    let test_app = spawn_app().await;
    let filename = "test.txt";

    let client = reqwest::Client::new();
    let image_part = reqwest::multipart::Part::text("this is some text")
        .file_name(filename.clone())
        .mime_str("text/plain")
        .unwrap();
    let form = reqwest::multipart::Form::new()
        .text("post-editor", text_body.clone())
        .part("image", image_part);
    let resp = client
        .post(&format!("{}/submit_post", &test_app.address))
        .multipart(form)
        .send()
        .await
        .expect("failed to reach route submit_post");

    assert_eq!(200, resp.status().as_u16());

    let saved_post = sqlx::query!("SELECT body, image FROM post")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved_post.body, text_body);
    assert!(saved_post.image.unwrap().contains(filename));
}

//#[actix_rt::test]
//async fn missing_field_user_add_send_400() {
//    let test_app = spawn_app().await;
//    let client = Client::new();
//    let bodies = [
//        (
//            "email=pasta%40pasta.com&password=he!he11o&password_ver=he!he11o",
//            "missing username",
//        ),
//        (
//            "username=lilpasta&password=he!he11o&password_ver=he!he11o",
//            "missing email",
//        ),
//        (
//            "username=lilpasta&email=pasta%40pasta.com&password_ver=he!he11o",
//            "missing password",
//        ),
//        (
//            "username=lilpasta&email=pasta%40pasta.com&password=he!he11o",
//            "missing password_ver",
//        ),
//    ];
//
//    for (b, e) in bodies {
//        let resp = client
//            .post(&format!("{}/add_user", &test_app.address))
//            .header("Content-Type", "application/x-www-form-urlencoded")
//            .body(b)
//            .send()
//            .await
//            .expect("failed to reach route add_user");
//        assert_eq!(400, resp.status().as_u16(), "failed to fail with {}", e);
//    }
//}
//
//#[actix_rt::test]
//async fn fields_present_invalid_submissions() {
//    let test_app = spawn_app().await;
//    let client = Client::new();
//    let bodies = [
//        (
//            "username=fn%20malware()%20->%20mal&email=pasta%40pasta.com&password=he!he11o&password_ver=he!he11o",
//            "invalid username",
//        ),
//        (
//            "username=lilpasta&email=pasta.com&password=he!he11o&password_ver=he!he11o",
//            "invalid email",
//        ),
//        (
//            "username=lilpasta&email=pasta%40pasta.com&password=he11o&password_ver=he11o",
//            "invalid password",
//        ),
//        (
//            "username=lilpasta&email=pasta%40pasta.com&password=he!he11o&password_ver=he!heoo11o",
//            "mismatching password_ver",
//        ),
//    ];
//
//    for (b, e) in bodies {
//        let resp = client
//            .post(&format!("{}/add_user", &test_app.address))
//            .header("Content-Type", "application/x-www-form-urlencoded")
//            .body(b)
//            .send()
//            .await
//            .expect("failed to reach route add_user");
//        assert_eq!(400, resp.status().as_u16(), "failed to fail with {}", e);
//    }
//}
