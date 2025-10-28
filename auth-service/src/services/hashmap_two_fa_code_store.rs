use std::collections::HashMap;

use crate::domain::{
    LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError,
    Email,
};

#[derive(Default, Clone)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email).ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound).map(|_| ())
        
    }
    
    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes.get(email).cloned().ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // adjust to your module structure
    use std::collections::HashMap;
    
    fn make_store() -> HashmapTwoFACodeStore {
        HashmapTwoFACodeStore {
            codes: HashMap::new(),
        }
    }
    
    fn make_sample_data() -> (Email, LoginAttemptId, TwoFACode) {
        (
            Email::parse("user@example.com").expect("Must be valid email"),
            LoginAttemptId::default(),
            TwoFACode::default(),
        )
    }
    
    #[tokio::test]
    async fn add_and_get_code_success() {
        let mut store = make_store();
        let (email, attempt_id, code) = make_sample_data();
    
        store.add_code(email.clone(), attempt_id.clone(), code.clone())
            .await
            .expect("add_code should succeed");
    
        let (stored_attempt_id, stored_code) = store
            .get_code(&email)
            .await
            .expect("get_code should succeed");
    
        assert_eq!(stored_attempt_id, attempt_id);
        assert_eq!(stored_code, code);
    }
    
    #[tokio::test]
    async fn get_code_not_found() {
        let store = make_store();
        let email = Email::parse("notfound@example.com").expect("Must be valid email");
    
        let err = store.get_code(&email).await.unwrap_err();
        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }
    
    #[tokio::test]
    async fn remove_code_success() {
        let mut store = make_store();
        let (email, attempt_id, code) = make_sample_data();
    
        store.add_code(email.clone(), attempt_id, code).await.unwrap();
    
        store.remove_code(&email).await.expect("remove_code should succeed");
    
        let err = store.get_code(&email).await.unwrap_err();
        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }
    
    #[tokio::test]
    async fn remove_code_not_found() {
        let mut store = make_store();
        let email = Email::parse("unknown@example.com").expect("Must be valid email");
    
        let err = store.remove_code(&email).await.unwrap_err();
        assert!(matches!(err, TwoFACodeStoreError::LoginAttemptIdNotFound));
    }
    
    #[tokio::test]
    async fn overwrite_existing_code() {
        let mut store = make_store();
        let (email, attempt_id1, code1) = make_sample_data();
        let attempt_id2 = LoginAttemptId::default(); // new UUID
        let code2 = TwoFACode::parse("999999".to_owned()).expect("Must be valid 2FA code");
    
        store.add_code(email.clone(), attempt_id1.clone(), code1.clone()).await.unwrap();
        store.add_code(email.clone(), attempt_id2.clone(), code2.clone()).await.unwrap();
    
        let (stored_attempt_id, stored_code) = store.get_code(&email).await.unwrap();
        assert_eq!(stored_attempt_id, attempt_id2);
        assert_eq!(stored_code, code2);
    }
}
