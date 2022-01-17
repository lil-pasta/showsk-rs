use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error("Password attempt failed: Incorrect password")]
    IncorrectPassword,
    #[error("Error fetching username")]
    IncorrectUsername,
    #[error("Password attempt failed: System error")]
    SystemError(#[from] anyhow::Error),
}

#[derive(serde::Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: Secret<String>,
}

impl UserCredentials {
    // check if the username and password the user entered is valid
    #[tracing::instrument(name = "Validating credentials", skip(self, pool))]
    pub async fn validate_credentials(
        &self,
        pool: &PgPool,
    ) -> Result<uuid::Uuid, AuthorizationError> {
        let mut user_id = None;
        let mut expected_password_string: Secret<String> = Secret::new(String::new());

        // since sqlx will return None (not an error) if the search doesnt return any matches
        // you need to account for that possibility.
        if let Some((uid, pwd)) = get_user_credentials(pool, &self.username).await? {
            user_id = Some(uid);
            expected_password_string = pwd;
        }

        // pwd from db into a PasswordHash
        let expected_password_hash = PasswordHash::new(expected_password_string.expose_secret())
            .map_err(|e| AuthorizationError::IncorrectPassword)?;

        match user_id {
            Some(_) => match check_hash(expected_password_hash, &self.password) {
                Ok(true) => Ok(user_id.unwrap()),
                Ok(false) => Err(AuthorizationError::IncorrectPassword),
                Err(e) => Err(AuthorizationError::SystemError(e.into())),
            },
            None => Err(AuthorizationError::IncorrectUsername),
        }
    }
}

#[tracing::instrument(name = "Validating credentials", skip(test))]
pub fn check_hash(expected: PasswordHash, test: &Secret<String>) -> Result<bool, anyhow::Error> {
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(test.expose_secret().as_bytes(), &expected)
        .is_ok())
}

#[tracing::instrument(name = "Validating credentials")]
pub async fn get_user_credentials(
    pool: &PgPool,
    username: &String,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username=$1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context(format!("could not fine username: {} in database", username))?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));
    Ok(row)
}
