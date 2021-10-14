use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use derive_more::{Display, Error};
use unicode_segmentation::UnicodeSegmentation;

// custom error type for an invalid password
// this is necessary to handle the multiple possible
// error return types in the parse function below
#[derive(Debug, Display, Error)]
pub enum PasswordError {
    #[display(fmt = "Password does not meet requirements")]
    InvalidPassword,
    #[display(fmt = "Passwords dont match")]
    PasswordVerFail,
    #[display(fmt = "Error generating random numbers")]
    SystemError,
}

#[derive(Debug)]
pub struct PassHash(String);

impl PassHash {
    pub fn parse(pw: String, pwv: String) -> Result<PassHash, PasswordError> {
        let too_long = pw.graphemes(true).count() > 1000;
        let whitespace_or_empty = pw.trim().is_empty();
        let too_short = pw.graphemes(true).count() < 8;
        let special_chars = [
            ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
            ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', '"',
        ];
        let contains_special_char = pw.chars().any(|c| special_chars.contains(&c));
        let password_match = pw == pwv;

        // set up hash boy
        let salt = SaltString::generate(&mut OsRng);
        // you need to fix this error prop issue here. using invalidpassword
        // for this is absolutely garbo
        if password_match {
            if too_long || whitespace_or_empty || too_short || !contains_special_char {
                Err(PasswordError::InvalidPassword)
            } else {
                let hash = PassHash::hash_password(pw.as_bytes(), &salt).unwrap();
                Ok(Self(hash))
            }
        } else {
            Err(PasswordError::PasswordVerFail)
        }
    }

    fn hash_password(pw: &[u8], salt: &SaltString) -> Result<String, PasswordError> {
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(pw, &salt)
            .map_err(|_e| PasswordError::SystemError)?
            .to_string())
    }

    pub fn check_hash(enc: &PasswordHash, pwd: &[u8]) -> Result<bool, argon2::Error> {
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(pwd, enc).is_ok())
    }
}

impl AsRef<str> for PassHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user_password::PassHash;
    use argon2::password_hash::PasswordHash;
    use claim::{assert_err, assert_ok};

    #[test]
    fn valid_password_verify_hash() {
        let password: String = String::from("heLl01!3 2");
        let password_ver: String = String::from("heLl01!3 2");
        assert_ok!(PassHash::parse(password.clone(), password_ver.clone()));

        let hash = PassHash::parse(password.clone(), password_ver.clone())
            .unwrap()
            .0;
        assert_ok!(PassHash::check_hash(
            &PasswordHash::new(&hash).unwrap(),
            &password.as_bytes()
        ));
    }

    #[test]
    fn missing_special_char() {
        let password: String = String::from("password");
        assert_err!(PassHash::parse(password.clone(), password));
    }

    #[test]
    fn all_white_space() {
        let password: String = String::from("         ");
        assert_err!(PassHash::parse(password.clone(), password));
    }

    #[test]
    fn empty_string() {
        let password: String = String::from("");
        assert_err!(PassHash::parse(password.clone(), password));
    }

    #[test]
    fn password_too_long() {
        let password: String = String::from("a".repeat(1001));
        assert_err!(PassHash::parse(password.clone(), password));
    }

    #[test]
    fn password_ver_no_match() {
        let password: String = String::from("he223 12!");
        let password_ver: String = String::from("hello");
        assert_err!(PassHash::parse(password, password_ver));
    }
}
