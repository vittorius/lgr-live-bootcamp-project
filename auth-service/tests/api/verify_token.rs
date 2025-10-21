use auth_service::domain::{BannedTokenStore, Email};
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
        .post_verify_token(&json!({ "token": TestApp::get_valid_auth_token(&email) }))
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

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;
    let token = TestApp::get_valid_auth_token(
        &Email::parse(&TestApp::get_random_email()).expect("Failed to parse email"),
    );

    let response = app
        .post_verify_token(&json!({
            "token": token
        }))
        .await;
    assert_eq!(response.status().as_u16(), 200);
    
    app.banned_token_store.write().await.add_token(token.clone()).await.expect("Failed to ban token");
    
    let response = app
        .post_verify_token(&json!({
            "token": token
        }))
        .await;
    assert_eq!(response.status().as_u16(), 401);
}
