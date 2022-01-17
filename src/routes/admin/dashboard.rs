use crate::{
    session_state::TypedSession,
    startup::AppData,
    utils::{e500, get_username},
};
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use anyhow::Context;

#[get("/admin/dashboard")]
pub async fn admin_dashboard(
    data: web::Data<AppData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let mut ctx = tera::Context::new();

    // TODO: abstract the below portion of code
    // this snippit validates a user as logged in and gets their username while protecting
    // the route from an unvalidated user.
    let username = if let Some(uid) = session.get_user_id().map_err(e500)? {
        get_username(uid, &data.db_pool).await.map_err(e500)?
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };
    ctx.insert("user", &username);
    let template = data.template.render("admin/dashboard.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(template))
}
