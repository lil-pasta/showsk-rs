use secrecy::{ExposeSecret, Secret};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("Password attempt failed: Incorrect password")]
    IncorrectPassword,
    #[error("Password attempt failed: System error")]
    SystemError,
}

pub struct UserCredentials {
    username: String,
    password: Secret<String>,
}

impl UserCredentials {
    pub fn new(username: &str, pwd: &str) -> Result<Self, AuthorizationError> {
        Ok((Self {
            username: String::from("username"),
            password: Secret::new(String::from("Hello")),
        }))
    }
}
