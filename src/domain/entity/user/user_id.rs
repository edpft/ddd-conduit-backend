use uuid::Uuid;

#[derive(Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> UserId {
        UserId(uuid)
    }
}

impl TryInto<UserId> for &str {
    type Error = UserIdError;

    fn try_into(self) -> Result<UserId, Self::Error> {
        let uuid = Uuid::try_parse(self)?;
        let user_id = uuid.into();
        Ok(user_id)
    }
}

impl TryInto<UserId> for String {
    type Error = UserIdError;

    fn try_into(self) -> Result<UserId, Self::Error> {
        self.as_str().try_into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserIdError {
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}
