use std::fmt::Display;

use argon2::{password_hash::Salt, PasswordHash};
use uuid::Uuid;

use super::{Email, Following, Password, PasswordError, UserError, Username, UsernameList};

type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User<'a> {
    id: Uuid,
    username: Username,
    email: Email,
    password_hash: PasswordHash<'a>,
    bio: Option<String>,
    image: Option<String>,
    following: Option<Following<'a>>,
}

impl<'a> User<'a> {
    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn change_email(mut self, new_email: Email) -> Result<Self> {
        self.email = new_email;
        Ok(self)
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn change_username(mut self, new_username: Username) -> Result<Self> {
        self.username = new_username;
        Ok(self)
    }

    pub fn password_hash(&self) -> &PasswordHash<'a> {
        &self.password_hash
    }

    pub fn change_password(
        mut self,
        old_password: Password,
        new_password: Password,
        salt: impl Into<Salt<'a>>,
    ) -> Result<Self> {
        let salt = salt.into();
        let old_password_hash = old_password.hash(salt)?;
        if &old_password_hash != self.password_hash() {
            let password_error = PasswordError::DoesNotMatch;
            let user_error = UserError::Password(password_error);
            return Err(user_error);
        };
        let new_password_hash = new_password.hash(salt)?;
        self.password_hash = new_password_hash;
        Ok(self)
    }

    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    pub fn change_image(mut self, new_image: Option<impl Into<String>>) -> Self {
        self.image = new_image.map(|string| string.into());
        self
    }

    pub fn bio(&self) -> Option<&str> {
        self.bio.as_deref()
    }

    pub fn change_bio(mut self, new_bio: Option<impl Into<String>>) -> Self {
        self.bio = new_bio.map(|string| string.into());
        self
    }

    pub fn follow(mut self, username: &Username) -> Result<Self> {
        let Some(following) = self.following.as_mut() else {
            let following = Following::new(username);
            self.following = Some(following);
            return Ok(self);
        };
        following.add(username);
        Ok(self)
    }

    pub fn unfollow(mut self, username: &Username) -> Result<Self> {
        let Some(following) = self.following.as_mut() else {
            return None;
        };
        following.remove(username);
        Ok(self)
    }
}

impl<'a> Display for User<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
