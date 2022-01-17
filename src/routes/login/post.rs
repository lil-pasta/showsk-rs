use crate::authorization::{AuthorizationError, UserCredentials};
use crate::session_state::TypedSession;
use crate::startup::AppData;
use actix_web::error::InternalError;
use actix_web::{http::header::LOCATION, post, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Username and password are incorrect")]
    IncorrectLogin(#[source] anyhow::Error),
    #[error("Something went wrong")]
    SystemError(#[from] anyhow::Error),
}

// check login creds and redirect on success/fail
#[post("/login")]
#[tracing::instrument(name = "Attempt login", skip(session, user, data), fields(username=%user.username))]
pub async fn login_validate(
    user: web::Form<UserCredentials>,
    data: web::Data<AppData>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let pool = &data.db_pool;

    // the successful flow will need to be refactored to create and store session data
    // in a more elegant/secure way than storing a user_id cookie
    match user.validate_credentials(&pool).await {
        Ok(uid) => {
            session.renew();
            session
                .insert_user(uid)
                .map_err(|e| login_redirect(LoginError::SystemError(e.into())))?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthorizationError::IncorrectUsername => LoginError::IncorrectLogin(e.into()),
                AuthorizationError::IncorrectPassword => LoginError::IncorrectLogin(e.into()),
                _ => LoginError::SystemError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

// reuse the failure logic in case of error
fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}
