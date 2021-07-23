// email validation is hard, let someone else do it for you
use validator::validate_email;

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(email: String) -> Result<UserEmail, String> {
        match validate_email(&email) {
            true => Ok(Self(email)),
            false => Err(format!("{} is not a valid email", email)),
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::domain::user_email::UserEmail;
    use claim::{assert_err, assert_ok};

    #[test]
    fn email_is_valid() {
        let email: String = String::from("thisis.anemail@email.com");
        assert_ok!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_at_sign() {
        let email: String = String::from("thisemail.com");
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_domain() {
        let email: String = String::from("thisemail");
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_missing_user() {
        let email: String = String::from("@fakemail.com");
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_invalid_whitespace() {
        let email: String = String::from("      ");
        assert_err!(UserEmail::parse(email));
    }

    #[test]
    fn email_invalid_empty() {
        let email: String = String::from("");
        assert_err!(UserEmail::parse(email));
    }
}
