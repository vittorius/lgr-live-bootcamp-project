use auth_service::Application;
use serde_json::json;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

const FAILED_TO_EXECUTE_REQUEST: &str = "Failed to execute request";

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
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

    pub async fn signup(
        &self,
        email: &str,
        password: &str,
        requires_2fa: bool,
    ) -> reqwest::Response {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(&json!({
                "email": email,
                "password": password,
                "requires2FA": requires_2fa,
            }))
            .send()
            .await
            .expect(FAILED_TO_EXECUTE_REQUEST)
    }

    pub async fn login(&self, email: &str, password: &str) -> reqwest::Response {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(&json!({
                "email": email,
                "password": password,
            }))
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
