use super::{username_list::UsernameListError, Username, UsernameList};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Following<'a> {
    current: Option<UsernameList<'a>>,
    to_add: Option<UsernameList<'a>>,
    to_remove: Option<UsernameList<'a>>,
}

impl<'a> Following<'a> {
    pub fn new(username: &Username) -> Self {
        let mut to_add = UsernameList::new();
        to_add.add(username);
        Self {
            to_add: Some(to_add),
            ..Default::default()
        }
    }

    pub fn add(mut self, username: &Username) -> Result<Self, UsernameListError> {
        let Some(to_add) = self.to_add else {
            let mut to_add = UsernameList::new();
            to_add.add(username)?;
            self.to_add = Some(to_add);
            return Ok(self);
        };
        let new_to_add = to_add.add(&username)?;
        self.to_add = Some(new_to_add);
        Ok(self)
    }

    pub fn remove(mut self, username: &Username) -> Result<Self, UsernameListError> {
        let Some(to_remove) = self.to_remove else {
            let mut to_remove = UsernameList::new();
            to_remove.add(username)?;
            self.to_remove = Some(to_remove);
            return Ok(self);
        };
        let new_to_remove = to_remove.add(&username)?;
        self.to_remove = Some(new_to_remove);
        Ok(self)
    }

    pub fn currently_following(&self) -> Option<&UsernameList> {
        match (self.current, self.to_add, self.to_remove) {
            (None, None, None) => None,
            (None, None, Some(to_remove)) => todo!(),
            (None, Some(to_add), None) => Some(&to_add),
            (None, Some(to_add), Some(to_remove)) => {
                let new_current: UsernameList = to_add.difference(&to_remove);
                Some(&new_current)
            }
            (Some(current), None, None) => Some(&current),
            (Some(current), None, Some(to_remove)) => {
                let new_current: UsernameList = current.difference(&to_remove);
                Some(&new_current)
            }
            (Some(current), Some(to_add), None) => {
                let new_current: UsernameList = current.union(&to_add);
                Some(&new_current)
            }
            (Some(current), Some(to_add), Some(to_remove)) => {
                let new_current: UsernameList = current.union(&to_add).difference(&to_remove);
                Some(&new_current)
            }
        }
    }
}
