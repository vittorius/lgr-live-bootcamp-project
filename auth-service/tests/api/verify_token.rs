use auth_service::{domain::Email, utils::auth::generate_auth_cookie};
use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(&json!({})).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let email = Email::parse(&TestApp::get_random_email()).expect("Failed to parse email");

    let response = app
        .post_verify_token(&json!({
            "token": generate_auth_cookie(&email).expect("Failed to generate auth cookie").value()
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let response = app
        .post_verify_token(&json!({
            "token": "foobar"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}
