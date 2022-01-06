use crate::startup::AppData;
use actix_web::{get, web, HttpResponse};
use tera::Context;

// serve the login form/page
#[get("/login")]
pub async fn login_form(data: web::Data<AppData>) -> HttpResponse {
    let ctx = Context::new();
    let template = data.template.render("login/login.html", &ctx).unwrap();
    HttpResponse::Ok().body(template)
}
