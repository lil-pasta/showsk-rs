use argon2::{self, Config};
use derive_more::{Display, Error};
use ring::rand;
use ring::rand::SecureRandom;
use unicode_segmentation::UnicodeSegmentation;

// custom error type for an invalid password
// this is necessary to handle the multiple possible
// error return types in the parse function below
#[derive(Debug, Display, Error)]
pub enum PasswordError {
    #[display(fmt = "Password does not meet requirements")]
    InvalidPassword,
    #[display(fmt = "Error generating random numbers")]
    SystemError,
}

#[derive(Debug)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn parse(pw: String) -> Result<PasswordHash, PasswordError> {
        let too_long = pw.graphemes(true).count() > 1000;
        let whitespace_or_empty = pw.trim().is_empty();
        let too_short = pw.graphemes(true).count() < 8;
        let special_chars = [
            ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
            ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', '"',
        ];
        let contains_special_char = pw.chars().any(|c| special_chars.contains(&c));

        // set up hash boy
        let rng = rand::SystemRandom::new();
        let mut salt = [0u8; 16];
        let conf = Config::default();
        // you need to fix this error prop issue here. using invalidpassword
        // for this is absolutely garbo
        rng.fill(&mut salt)
            .map_err(|_| PasswordError::SystemError)?;

        if too_long || whitespace_or_empty || too_short || !contains_special_char {
            Err(PasswordError::InvalidPassword)
        } else {
            let hash = PasswordHash::hash_password(&pw.as_bytes(), &salt, &conf).unwrap();
            Ok(Self(hash))
        }
    }

    pub fn hash_password(pw: &[u8], salt: &[u8], conf: &Config) -> Result<String, argon2::Error> {
        argon2::hash_encoded(pw, salt, conf)
    }

    pub fn check_hash(enc: &str, pwd: &[u8]) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(enc, pwd)
    }
}

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user_password::PasswordHash;
    use claim::{assert_err, assert_ok};

    #[test]
    fn valid_password_verify_hash() {
        let password: String = String::from("heLl01!3 2");
        assert_ok!(PasswordHash::parse(password.clone()));
        assert_ok!(PasswordHash::check_hash(
            &PasswordHash::parse(password.clone()).unwrap().as_ref(),
            &password.as_bytes()
        ));
    }

    #[test]
    fn missing_special_char() {
        let password: String = String::from("password");
        assert_err!(PasswordHash::parse(password));
    }

    #[test]
    fn all_white_space() {
        let password: String = String::from("         ");
        assert_err!(PasswordHash::parse(password));
    }

    #[test]
    fn empty_string() {
        let password: String = String::from("");
        assert_err!(PasswordHash::parse(password));
    }

    #[test]
    fn password_too_long() {
        let password: String = String::from("a".repeat(1001));
        assert_err!(PasswordHash::parse(password));
    }
}
