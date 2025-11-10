use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};
use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.tokens
            .insert(token)
            .then_some(())
            .ok_or(BannedTokenStoreError::AlreadyExists)
    }

    async fn token_exists(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::default();

        // Test adding new token
        let result = store.add_token("token1".to_string()).await;
        assert!(result.is_ok());

        // Test adding duplicate token
        let result = store.add_token("token1".to_string()).await;
        assert!(matches!(result, Err(BannedTokenStoreError::AlreadyExists)));
    }

    #[tokio::test]
    async fn test_token_exists() {
        let mut store = HashsetBannedTokenStore::default();

        // Test non-existent token
        assert!(!store.token_exists("token1").await.expect("Failed to check token existence"));

        // Add token and test existence
        store.add_token("token1".to_string()).await.expect("Failed to add token");
        assert!(store.token_exists("token1").await.expect("Failed to check token existence"));

        // Test different non-existent token
        assert!(!store.token_exists("token2").await.expect("Failed to check token existence"));
    }
}
