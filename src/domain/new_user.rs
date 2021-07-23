use crate::domain::user_email::UserEmail;
use crate::domain::user_password::PasswordHash;
use crate::domain::username::Username;

pub struct NewUser {
    email: UserEmail,
    username: Username,
    password: PasswordHash,
}

impl NewUser {
    pub fn new(email: String, username: String, password: String) -> Result<NewUser, String> {
        let username: Username = Username::parse(username)?;
        let user_email: UserEmail = UserEmail::parse(email)?;
        let user_password: PasswordHash =
            PasswordHash::parse(password).map_err(|e| e.to_string())?;

        Ok(NewUser {
            email: user_email,
            username: username,
            password: user_password,
        })
    }
}
