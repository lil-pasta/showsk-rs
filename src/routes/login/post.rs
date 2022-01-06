use crate::startup::AppData;
use actix_web::{get, post, web, HttpResponse};
use tera::Context;

// check login creds and redirect on success/fail
#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}
