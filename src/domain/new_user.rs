use crate::domain::user_email::UserEmail;
use crate::domain::user_password::PasswordHash;
use crate::domain::username::Username;

#[derive(Debug)]
pub struct NewUser {
    pub email: UserEmail,
    pub username: Username,
    pub password_hash: PasswordHash,
}

impl NewUser {
    pub fn new(
        email: String,
        username: String,
        password: String,
        password_ver: String,
    ) -> Result<NewUser, String> {
        let user_name: Username = Username::parse(username)?;
        let user_email: UserEmail = UserEmail::parse(email)?;
        let user_password: PasswordHash =
            PasswordHash::parse(password, password_ver).map_err(|e| e.to_string())?;

        Ok(NewUser {
            email: user_email,
            username: user_name,
            password_hash: user_password,
        })
    }
}
