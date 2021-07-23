use crate::domain::user_email::UserEmail;
use crate::domain::username::Username;

pub struct NewUser {
    email: UserEmail,
    username: Username,
}

impl NewUser {
    pub fn new(email: String, username: String) -> Result<NewUser, String> {
        let username: Username = Username::parse(username)?;
        let user_email: UserEmail = UserEmail::parse(email)?;

        Ok(NewUser {
            email: user_email,
            username: username,
        })
    }
}
