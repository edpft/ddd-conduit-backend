#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username(String);

impl Username {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryInto<Username> for String {
    type Error = UsernameError;

    fn try_into(self) -> Result<Username, Self::Error> {
        let username_length = self.len();
        match username_length {
            0 => {
                let error = UsernameError::Empty;
                Err(error)
            }
            65.. => {
                let error = UsernameError::TooLong;
                Err(error)
            }
            _ => {
                let username = Username(self);
                Ok(username)
            }
        }
    }
}

impl TryInto<Username> for &str {
    type Error = UsernameError;

    fn try_into(self) -> Result<Username, Self::Error> {
        let username_string = self.to_string();
        username_string.try_into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UsernameError {
    #[error("Username cannot be empty")]
    Empty,
    #[error("Username cannot be more than 64 characters")]
    TooLong,
}
