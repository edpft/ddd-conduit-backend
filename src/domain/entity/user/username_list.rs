use std::collections::HashSet;

use super::Username;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UsernameList<'a>(HashSet<&'a Username>);

impl<'a> UsernameList<'a> {
    pub fn new() -> Self {
        let mut username_list = HashSet::new();
        UsernameList(username_list)
    }

    pub fn add(mut self, username: &Username) -> Result<UsernameList, UsernameListError> {
        if self.contains(username) {
            let error = UsernameListError::AlreadyExists;
            Err(error)
        } else {
            self.0.insert(username);
            Ok(self)
        }
    }

    pub fn remove(mut self, username: &Username) -> Result<UsernameList, UsernameListError> {
        if !self.contains(username) {
            let error = UsernameListError::NotFound;
            Err(error)
        } else {
            self.0.remove(username);
            Ok(self)
        }
    }

    pub fn contains(&self, username: &Username) -> bool {
        self.0.contains(&username)
    }

    pub fn union(&self, other: &Self) -> Self {
        let self_hashset = self.0;
        let other_hashset = other.0;
        self_hashset.union(&other_hashset).cloned().collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        let self_hashset = self.0;
        let other_hashset = other.0;
        self_hashset.difference(&other_hashset).cloned().collect()
    }
}

impl<'a> FromIterator<&'a Username> for UsernameList<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Username>>(usernames: T) -> Self {
        let mut username_list = UsernameList::new();
        for username in usernames {
            username_list.add(username);
        }
        username_list
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UsernameListError {
    #[error("Username already in list")]
    AlreadyExists,
    #[error("Username not found in list")]
    NotFound,
}
