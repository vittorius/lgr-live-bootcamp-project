use crate::domain::{Email, Password};

use super::User;
use async_trait::async_trait;
use color_eyre::eyre::{eyre, Report, Result as EyreResult, WrapErr};
use rand::Rng;
use secrecy::Secret;
use thiserror::Error;
use uuid::Uuid;

#[async_trait]
pub trait UserStore: Send + Sync + 'static {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[async_trait]
pub trait BannedTokenStore: Send + Sync + 'static {
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Already exists")]
    AlreadyExists,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait]
pub trait TwoFACodeStore: Send + Sync + 'static {
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

#[derive(Error, Debug)]
pub enum TwoFACodeStoreError {
    #[error("Login Attempt ID not found")]
    LoginAttemptIdNotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    // TODO: add unit tests later
    pub fn parse(id: String) -> EyreResult<Self> {
        let parsed_id = uuid::Uuid::parse_str(&id).wrap_err("Invalid login attempt id")?; // Updated!

        Ok(Self(parsed_id.to_string()))
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
    // TODO: add unit tests later
    pub fn parse(code: String) -> EyreResult<Self> {
        let code_as_u32 = code.parse::<u32>().wrap_err("Invalid 2FA code")?; // Updated!

        if (100_000..=999_999).contains(&code_as_u32) {
            Ok(Self(code))
        } else {
            Err(eyre!("Invalid 2FA code")) // Updated!
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut code = String::new();
        for _ in 0..6 {
            code.push_str(&rng.gen_range(0..10).to_string());
        }
        Self(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_two_fa_code_is_valid() {
        for _ in 0..100 { // because this is random
            let code = TwoFACode::default();
            assert_eq!(code.0.len(), 6);
            assert!(code.0.chars().all(|c| c.is_ascii_digit()));
        }
    }
}
