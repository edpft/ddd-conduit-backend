use argon2::{password_hash::Salt, Argon2, PasswordHash, PasswordHasher};

use super::PasswordError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Password(String);

impl Password {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn hash<'a>(self, salt: impl Into<Salt<'a>>) -> Result<PasswordHash<'a>, PasswordError> {
        let salt = salt.into();
        let password_bytes = self.as_bytes();
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password_bytes, salt)?;
        Ok(password_hash)
    }
}

impl TryInto<Password> for String {
    type Error = PasswordError;

    fn try_into(self) -> Result<Password, Self::Error> {
        let password_length = self.len();
        match password_length {
            // Passwords shorter than 8 characters are considered to be weak.
            0..=7 => {
                let password_error = PasswordError::TooShort(password_length);
                Err(password_error)
            }
            // A common maximum length is 64 characters due to limitations in certain hashing algorithms.
            // It is important to set a maximum password length to prevent long password Denial of Service
            // attacks.
            65.. => {
                let password_error = PasswordError::TooLong(password_length);
                Err(password_error)
            }
            _ => {
                let password = Password(self);
                Ok(password)
            }
        }
    }
}

impl TryInto<Password> for &str {
    type Error = PasswordError;

    fn try_into(self) -> Result<Password, Self::Error> {
        let password_string = self.to_string();
        password_string.try_into()
    }
}
