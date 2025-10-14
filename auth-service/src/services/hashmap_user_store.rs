use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.users
            .insert(user.email.clone(), user)
            .map_or(Ok(()), |_| Err(UserStoreError::UserAlreadyExists))
    }

    // TODO: convert from &User to User if necessary
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            (user.email == email && user.password == password)
                .then_some(())
                .ok_or(UserStoreError::InvalidCredentials)
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user_new() {
        let mut store = HashmapUserStore::default();
        assert_eq!(store.users.len(), 0);

        let user = User::new("me", "pass", false);

        store.add_user(user.clone()).unwrap();
        assert_eq!(store.users.len(), 1);
    }

    #[test]
    fn test_add_user_if_exists() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);

        store.add_user(user.clone()).unwrap();
        assert_eq!(
            store.add_user(user.clone()),
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[test]
    fn test_get_user_if_exists() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);

        store.add_user(user.clone()).unwrap();
        assert_eq!(store.get_user(&user.email).unwrap(), &user);
    }

    #[test]
    fn test_get_user_if_not_exists() {
        let store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);

        assert_eq!(
            store.get_user(&user.email),
            Err(UserStoreError::UserNotFound)
        );
    }

    #[test]
    fn test_validate_user_valid() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);
        store.add_user(user).unwrap();

        assert!(store.validate_user("me", "pass").is_ok());
    }

    #[test]
    fn test_validate_user_invalid() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);
        store.add_user(user).unwrap();

        assert_eq!(
            store.validate_user("me", "not_my_pass"),
            Err(UserStoreError::InvalidCredentials)
        );
    }

    #[test]
    fn test_validate_user_if_not_exists() {
        let store = HashmapUserStore::default();

        assert_eq!(
            store.validate_user("me", "pass"),
            Err(UserStoreError::UserNotFound)
        );
    }
}
