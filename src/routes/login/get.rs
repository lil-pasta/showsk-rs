use crate::startup::AppData;
use actix_web::{get, web, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use tera::Context;

// serve the login form/page
#[get("/login")]
pub async fn login_form(data: web::Data<AppData>, flash: IncomingFlashMessages) -> HttpResponse {
    let mut ctx = Context::new();
    let mut errors = Vec::new();
    for m in flash.iter().filter(|m| m.level() == Level::Error) {
        errors.push(m.content());
    }

    ctx.insert("error", &errors);
    let template = data.template.render("login/login.html", &ctx).unwrap();
    HttpResponse::Ok().body(template)
}
