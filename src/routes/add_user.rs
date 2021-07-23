use crate::domain::new_user::NewUser;
use crate::startup::AppData;
use actix_web::{
    dev::HttpResponseBuilder, error, http::StatusCode, post, web, HttpResponse, Result,
};
use chrono::Utc;
use derive_more::{Display, Error};
use sqlx::PgPool;
use uuid::Uuid;

// custom error handler for the route
#[derive(Debug, Display, Error)]
pub enum NewUserError {
    #[display(fmt = "An internal error occured. Please try again later")]
    QueryError,
    #[display(fmt = "Error parsing submitted fields")]
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

pub async fn new_user_submission(
    data: web::Data<AppData>,
    form: web::Form<NewUserForm>,
) -> Result<HttpResponse, NewUserError> {
    // use your domain!
    // let new_user = NewUser::new(form.username, form.email, form.password, form.password_ver)
}

pub async fn insert_user(user: &NewUser, db_pool: &PgPool) -> Result<(), sqlx::Error> {}
