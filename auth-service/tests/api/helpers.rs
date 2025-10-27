use std::sync::Arc;

use auth_service::{
    app_state::{AppState, TokenStoreType},
    domain::Email,
    services::{HashmapUserStore, HashsetBannedTokenStore},
    utils::{auth::generate_auth_cookie, constants::test},
    Application,
};
use reqwest::cookie::Jar;
use serde::Serialize;
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token_store: TokenStoreType<HashsetBannedTokenStore>,
}

const FAILED_TO_EXECUTE_REQUEST: &str = "Failed to execute request";

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let app_state = AppState::new(user_store.clone(), banned_token_store.clone());

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            http_client,
            banned_token_store: banned_token_store.clone(),
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub fn get_random_email() -> String {
        format!("{}@example.com", Uuid::new_v4())
    }

    pub fn get_valid_auth_token(email: &Email) -> String {
        generate_auth_cookie(email)
            .expect("Failed to generate auth cookie")
            .value()
            .to_string()
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub async fn verify_2fa(
        &self,
        email: &str,
        login_attempt_id: &str,
        code_2fa: &str,
    ) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(&json!({
                "email": email,
                "loginAttemptId": login_attempt_id,
                "2FACode": code_2fa,
            }))
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }
}
