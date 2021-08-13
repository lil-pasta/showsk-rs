use crate::startup::AppData;
use actix_web::{get, web, HttpResponse, Responder};
use tera::Context;

// TODO: impliment posts on front page (home)
#[get("/")]
pub async fn index(data: web::Data<AppData>) -> impl Responder {
    let name = "User";
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let template = data.template.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(template)
}
