use crate::domain::{Email, Password};

use super::User;
use async_trait::async_trait;
use rand::Rng;
use uuid::Uuid;

#[async_trait]
pub trait UserStore: Clone + Send + Sync + 'static {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<&User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait]
pub trait BannedTokenStore: Clone + Send + Sync + 'static {
    async fn add_token(&mut self, token: String) -> Result<(), TokenStoreError>;
    async fn token_exists(&self, token: &str) -> bool;
}

#[derive(Debug)]
pub enum TokenStoreError {
    AlreadyExists,
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore: Clone + Send + Sync + 'static {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        Uuid::parse_str(&id)
            .map(|_| Self(id))
            .map_err(|_| "Invalid UUID".to_owned())
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            Err("Invalid 2FA code".to_owned())
        } else {
            Ok(Self(code))
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut code = String::default();
        (0..6).for_each(|_| code.push(rng.gen_range(0..10).into()));
        Self(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
