#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryInto<Email> for String {
    type Error = EmailError;

    fn try_into(self) -> Result<Email, Self::Error> {
        if !validator::validate_email(self) {
            let error = EmailError::NotValidHtml5(self);
            Err(error)
        } else {
            let email = Email(self);
            Ok(email)
        }
    }
}

impl TryInto<Email> for &str {
    type Error = EmailError;

    fn try_into(self) -> Result<Email, Self::Error> {
        let email_string = self.to_string();
        email_string.try_into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("{0} is not a valid HTML5 email address.")]
    NotValidHtml5(String),
}
