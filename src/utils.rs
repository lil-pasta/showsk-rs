use actix_web::{error::InternalError, http::header::LOCATION, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

// return internal server error
pub fn e500<E>(e: E) -> InternalError<E> {
    InternalError::from_response(e, HttpResponse::InternalServerError().finish())
}

pub fn see_other(dest: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, dest))
        .finish()
}

#[tracing::instrument(name = "get username", skip(pool))]
pub async fn get_username(uid: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1"#,
        uid
    )
    .fetch_one(pool)
    .await
    .context("failed to find a user with that uid")?;
    Ok(query.username)
}
