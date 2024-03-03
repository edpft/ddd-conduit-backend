use super::{email::EmailError, user_id::UserIdError, Email};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error(transparent)]
    Password(PasswordError),
    #[error(transparent)]
    Id(UserIdError),
    #[error(transparent)]
    Email(EmailError),
}

impl From<EmailError> for UserError {
    fn from(email_error: EmailError) -> Self {
        Self::Email(email_error)
    }
}

impl From<PasswordError> for UserError {
    fn from(password_error: PasswordError) -> Self {
        Self::Password(password_error)
    }
}

impl From<UserIdError> for UserError {
    fn from(user_id_error: UserIdError) -> Self {
        Self::Id(user_id_error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("Password must be at least 8 characters. Password length is {0}.")]
    TooShort(usize),
    #[error("Password cannot be more than 64 characters. Password length is {0}.")]
    TooLong(usize),
    #[error("password does not match")]
    DoesNotMatch,
    #[error(transparent)]
    Hashing(#[from] argon2::password_hash::Error),
}
