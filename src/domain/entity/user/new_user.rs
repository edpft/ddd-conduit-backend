use uuid::Uuid;

use super::{
    hash_password, Email, Password, PasswordHash, UserError, UserId, UserIdError, Username,
};

type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewUser {
    id: Uuid,
    username: String,
    email: Email,
    password_hash: PasswordHash,
}

impl NewUser {
    pub fn new(
        username: impl Into<Username>,
        email: impl TryInto<Email>,
        password: impl TryInto<Password>,
        salt: impl Into<String>,
        id: Option<impl TryInto<UserId>>,
    ) -> Result<Self> {
        let id = match id {
            None => UserId::new(),
            Some(id) => {
                let user_id: std::result::Result<UserId, UserIdError> = id.try_into();
                Ok(user_id)
            }
        };
        let username = username.try_into()?;
        let email = email.try_into()?;
        let password_hash = hash_password(password, salt)?;

        let user = Self {
            id,
            username,
            email,
            password_hash,
        };
        Ok(user)
    }
}
