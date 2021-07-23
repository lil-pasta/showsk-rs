use argon2::{self, Config, Variant};
use unicode_segmentation::UnicodeSegmentation;
use ring::rand;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use std::error;
use std::fmt;

// custom error type for an invalid password
// this is necessary to handle the multiple possible
// error return types in the parse function below
#[derive(Debug)]
pub struct InvalidPassword;

impl fmt::Display for InvalidPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid password")
    }
}

impl std::error::Error for InvalidPassword {}

#[derive(Debug)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn parse(pw: String) -> Result<PasswordHash, Box<dyn error::Error>> {
        let too_long = pw.graphemes(true).count() > 1000;
        let whitespace_or_empty = pw.trim().is_empty();
        let too_short = pw.graphemes(true).count() < 8;
        let special_chars = [' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', '"'];
        let contains_special_char = pw.chars().any(|c| special_chars.contains(&c));
        
        // set up hash boy
        let hash_conf = Config { variant: Variant::Argon2id };
        let rng = rand::SystemRandom::new();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt).map_err(|e| e.into())?;

        if too_long || whitespace_or_empty || too_short || !contains_special_char {
            Ok(PasswordHash::hash_password(&pw.as_bytes(), &salt, hash_conf).unwrap())
        } else {
            InvalidPassword.into()
        }
    }

    pub fn hash_password(pw: &[u8], salt: &[u8], config: &Config) -> Result<String, argon2::Error> {
        argon2::hash_encoded(pw, salt, config)
    }

    pub fn check_hash(enc: &str, pwd: &[u8]) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(enc, pwd)
    }
}

impl AsRef<str> for PasswordHash {
    pub fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user_password::PasswordHash;
    use claim::{assert_ok, assert_err};

    #[test]
    fn valid_password_verify_hash() {
        let password: String = String::from("heLl01!3 2");
        assert_ok!(PasswordHash::parse(password));
        assert_ok!(PasswordHash::check_hash(&PasswordHash::parse(password), &password.as_bytes());
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


