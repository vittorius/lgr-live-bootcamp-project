use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};
use async_trait::async_trait;
use secrecy::{ExposeSecret, Secret};

#[derive(Default, Clone)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        self.tokens
            .insert(token.expose_secret().clone())
            .then_some(())
            .ok_or(BannedTokenStoreError::AlreadyExists)
    }

    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::default();

        // Test adding new token
        let result = store.add_token("token1".to_owned().into()).await;
        assert!(result.is_ok());

        // Test adding duplicate token
        let result = store.add_token("token1".to_owned().into()).await;
        assert!(matches!(result, Err(BannedTokenStoreError::AlreadyExists)));
    }

    #[tokio::test]
    async fn test_token_exists() {
        let mut store = HashsetBannedTokenStore::default();

        // Test non-existent token
        assert!(!store.contains_token(&"token1".to_owned().into()).await.expect("Failed to check token existence"));

        // Add token and test existence
        store.add_token("token1".to_owned().into()).await.expect("Failed to add token");
        assert!(store.contains_token(&"token1".to_owned().into()).await.expect("Failed to check token existence"));

        // Test different non-existent token
        assert!(!store.contains_token(&"token2".to_owned().into()).await.expect("Failed to check token existence"));
    }
}
