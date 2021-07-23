use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Username(String);

impl Username {
    pub fn parse(uname: String) -> Result<Username, String> {
        let is_empty_or_whitespace = uname.trim().is_empty();

        let too_long = uname.graphemes(true).count() > 256;

        let forbidden_chars = [' ', ',', '.', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden = uname.chars().any(|c| forbidden_chars.contains(&c));

        if is_empty_or_whitespace || too_long || contains_forbidden {
            Err(format!("{} is an invalid username", uname))
        } else {
            Ok(Self(uname))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::username::Username;
    use claim::{assert_err, assert_ok};

    #[test]
    pub fn valid_username() {
        let username: String = String::from("PeteyPablo322");
        assert_ok!(Username::parse(username));
    }

    #[test]
    pub fn username_too_long() {
        let username: String = String::from("penis".repeat(100));
        assert_err!(Username::parse(username));
    }

    #[test]
    pub fn username_contains_forbidden() {
        let username: String = String::from("pub fn hacktheplanet() -> Destruction {}");
        assert_err!(Username::parse(username));
    }

    #[test]
    pub fn username_empty() {
        let username: String = String::new();
        assert_err!(Username::parse(username));
    }

    #[test]
    pub fn usern_is_whitespace() {
        let username: String = String::from("      ");
        assert_err!(Username::parse(username));
    }
}
