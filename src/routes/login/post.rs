use crate::authorization::{AuthorizationError, UserCredentials};
use crate::startup::AppData;
use actix_web::{http::StatusCode, post, web, HttpResponse, HttpResponseBuilder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Username and password are incorrect")]
    IncorrectLogin(#[source] anyhow::Error),
    #[error("Something went wrong")]
    SystemError(#[from] anyhow::Error),
}

impl actix_web::error::ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            LoginError::IncorrectLogin(_) => StatusCode::UNAUTHORIZED,
            LoginError::SystemError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// check login creds and redirect on success/fail
#[post("/login")]
#[tracing::instrument(name = "Attempt login", skip(user, data), fields(username=%user.username))]
pub async fn login_validate(
    user: web::Form<UserCredentials>,
    data: web::Data<AppData>,
) -> Result<HttpResponse, LoginError> {
    let pool = &data.db_pool;
    match user.validate_credentials(&pool).await {
        Ok(user_id) => {
            Ok(HttpResponse::Ok()
                .body(format!("creds successfully validated for user {}", user_id)))
        }
        Err(e) => {
            let e = match e {
                AuthorizationError::IncorrectUsername => LoginError::IncorrectLogin(e.into()),
                AuthorizationError::IncorrectPassword => LoginError::IncorrectLogin(e.into()),
                _ => LoginError::SystemError(e.into()),
            };
            Err(e)
        }
    }
}
