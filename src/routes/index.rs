use crate::startup::AppData;
use actix_web::{get, web, Error, HttpResponse};
use serde;
use sqlx;
use tera::Context;

#[derive(serde::Deserialize, serde::Serialize)]
struct Post {
    body: String,
    image: String,
    timestamp: String,
}

#[get("/")]
pub async fn index(data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let name = "User";
    let resp = sqlx::query!("SELECT body, image, timestmp FROM post")
        .fetch_all(&**data.db_pool)
        .await
        .unwrap();
    let mut posts: Vec<Post> = Vec::new();
    for item in resp {
        let post = Post {
            body: item.body,
            image: item.image.unwrap(),
            timestamp: item.timestmp.to_string(),
        };
        posts.push(post);
    }
    let mut ctx = Context::new();
    ctx.insert("name", name);
    ctx.insert("posts", &posts);
    let template = data.template.render("index/index.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(template))
}
