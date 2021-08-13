use crate::startup::AppData;
use actix_web::{get, web, HttpResponse, Responder};
use tera::Context;

#[get("/write_post")]
async fn write_post(data: web::Data<AppData>) -> impl Responder {
    let ctx = Context::new();
    let template = data.template.render("post.html", &ctx).unwrap();
    HttpResponse::Ok().body(template)
}
