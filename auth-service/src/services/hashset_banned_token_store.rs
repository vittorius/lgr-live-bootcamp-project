use std::collections::HashSet;

use crate::domain::{BannedTokenStore, TokenStoreError};
use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), TokenStoreError> {
        self.tokens
            .insert(token)
            .then_some(())
            .ok_or(TokenStoreError::AlreadyExists)
    }

    async fn token_exists(&self, token: &str) -> bool {
        self.tokens.contains(token)
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
        assert!(matches!(result, Err(TokenStoreError::AlreadyExists)));
    }

    #[tokio::test]
    async fn test_token_exists() {
        let mut store = HashsetBannedTokenStore::default();

        // Test non-existent token
        assert!(!store.token_exists("token1").await);

        // Add token and test existence
        store.add_token("token1".to_string()).await.unwrap();
        assert!(store.token_exists("token1").await);

        // Test different non-existent token
        assert!(!store.token_exists("token2").await);
    }
}
