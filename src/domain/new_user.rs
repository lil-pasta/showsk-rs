use crate::domain::user_email::UserEmail;
use crate::domain::user_password::PassHash;
use crate::domain::username::Username;

#[derive(Debug)]
pub struct NewUser {
    pub email: UserEmail,
    pub username: Username,
    pub hashed_password: PassHash,
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
        let hashed_password = PassHash::parse(password, password_ver).map_err(|e| e.to_string())?;

        Ok(NewUser {
            email: user_email,
            username: user_name,
            hashed_password,
        })
    }
}
