use crate::domain::new_user::NewUser;
use crate::startup::AppData;
use actix_web::{error, http::StatusCode, post, web, HttpResponse, HttpResponseBuilder, Result};
use chrono::Utc;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

// custom error handler for the route
#[derive(Debug, Error)]
pub enum NewUserError {
    #[error("An internal error occured. Please try again later")]
    QueryError,
    #[error("Error parsing submitted fields")]
    ParseError,
}

impl error::ResponseError for NewUserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            NewUserError::QueryError => StatusCode::INTERNAL_SERVER_ERROR,
            NewUserError::ParseError => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct NewUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_ver: String,
}

#[post("/add_user")]
#[tracing::instrument(
    name="adding a new user",
    skip(data, form),
    fields(
        email=%form.email,
        username=%form.username,
    )
)]
pub async fn add_user(
    data: web::Data<AppData>,
    form: web::Form<NewUserForm>,
) -> Result<HttpResponse, NewUserError> {
    // use your domain! now there is only a single access point
    // for the api which should greatly increase app security and reliability
    let new_user = NewUser::new(
        form.0.email,
        form.0.username,
        form.0.password,
        form.0.password_ver,
    )
    .map_err(|_| NewUserError::ParseError)?;
    insert_user(&new_user, &data.db_pool)
        .await
        .map_err(|_| NewUserError::QueryError)?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "adding a new user", skip(db_pool))]
pub async fn insert_user(user: &NewUser, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (user_id, email, username, password_hash, joined_on)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        user.email.as_ref(),
        user.username.as_ref(),
        user.hashed_password.as_ref(),
        Utc::now(),
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert query: {:?}", e);
        e
    })?;
    Ok(())
}
