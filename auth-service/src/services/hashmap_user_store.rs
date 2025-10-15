use async_trait::async_trait;
use std::collections::HashMap;

use crate::domain::{User, UserStore, UserStoreError};

#[derive(Default, Clone)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.users
            .insert(user.email.clone(), user)
            .map_or(Ok(()), |_| Err(UserStoreError::UserAlreadyExists))
    }

    // TODO: convert from &User to User if necessary
    async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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

    #[tokio::test]
    async fn test_add_user_new() {
        let mut store = HashmapUserStore::default();
        assert_eq!(store.users.len(), 0);

        let user = User::new("me", "pass", false);

        store.add_user(user.clone()).await.unwrap();
        assert_eq!(store.users.len(), 1);
    }

    #[tokio::test]
    async fn test_add_user_if_exists() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);

        store.add_user(user.clone()).await.unwrap();
        assert_eq!(
            store.add_user(user.clone()).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user_if_exists() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);

        store.add_user(user.clone()).await.unwrap();
        assert_eq!(store.get_user(&user.email).await.unwrap(), &user);
    }

    #[tokio::test]
    async fn test_get_user_if_not_exists() {
        let store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);

        assert_eq!(
            store.get_user(&user.email).await,
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user_valid() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);
        store.add_user(user).await.unwrap();

        assert!(store.validate_user("me", "pass").await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user_invalid() {
        let mut store = HashmapUserStore::default();
        let user = User::new("me", "pass", false);
        store.add_user(user).await.unwrap();

        assert_eq!(
            store.validate_user("me", "not_my_pass").await,
            Err(UserStoreError::InvalidCredentials)
        );
    }

    #[tokio::test]
    async fn test_validate_user_if_not_exists() {
        let store = HashmapUserStore::default();

        assert_eq!(
            store.validate_user("me", "pass").await,
            Err(UserStoreError::UserNotFound)
        );
    }
}
