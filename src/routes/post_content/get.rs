use crate::{session_state::TypedSession, startup::AppData, utils::e500};
use actix_web::{get, http::header::LOCATION, web, HttpResponse};
use tera::Context;

#[get("/write_post")]
async fn write_post(
    data: web::Data<AppData>,
    session: TypedSession,
) -> Result<HttpResponse, actix_web::Error> {
    let _userid = if let Some(uid) = session.get_user_id().map_err(e500)? {
        uid
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };

    let ctx = Context::new();
    let template = data.template.render("post/post.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(template))
}
