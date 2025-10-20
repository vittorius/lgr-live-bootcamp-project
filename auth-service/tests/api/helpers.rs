use std::sync::Arc;

use auth_service::{app_state::AppState, services::HashmapUserStore, Application};
use serde::Serialize;
use serde_json::json;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

const FAILED_TO_EXECUTE_REQUEST: &str = "Failed to execute request";

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashmapUserStore::default();
        let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client,
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

    pub async fn logout(&self, jwt_token: &str) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout?jwt={}", &self.address, jwt_token))
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub async fn verify_token(&self, jwt_token: &str) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(&json!({
                "token": jwt_token,
            }))
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }
}
