use crate::helpers::TestApp;
use auth_service::{
    domain::Email,
    routes::{LoginResponse, SignupResponse, TwoFactorAuthResponse},
    utils::constants::JWT_COOKIE_NAME,
};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;
    let email = TestApp::get_random_email();
    let email_value = Email::parse(&email).expect("Must be valid email");

    let signup_body = serde_json::json!({
        "email": email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": email,
        "password": "password123",
    });

    let TwoFactorAuthResponse {
        login_attempt_id, ..
    } = app
        .post_login(&login_body)
        .await
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Must deserialize to TwoFactorAuthResponse");

    let (_, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email_value)
        .await
        .expect("2FA code must be present for email");

    let response = app.post_verify_2fa(&json!({ "email": email, "loginAttemptId": login_attempt_id, "2FACode": two_fa_code.as_ref() })).await;
    assert_eq!(response.status(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let test_cases = [
        json!({"email": null, "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": null, "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": null}),
        json!({"loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "123456"}),
        json!({"email": "me@example.com", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "12345678-1234-1234-1234-123456789012"}),
    ];

    for test_case in test_cases {
        let app = TestApp::new().await;
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status(), 422);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let test_cases = [
        json!({"email": "invalid-email", "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "invalid-uuid", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "invalid-code"}),
    ];

    for test_case in test_cases {
        let app = TestApp::new().await;
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status(), 400);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let email = TestApp::get_random_email();
    let email_value = Email::parse(&email).unwrap();
    let password = "password".to_owned();

    app.post_signup(&json!({
        "email": email,
        "password": password,
        "requires2FA": true
    }))
    .await
    .json::<SignupResponse>()
    .await
    .expect("Must deserialize to SignupResponse");

    let TwoFactorAuthResponse {
        login_attempt_id, ..
    } = app
        .post_login(&json!({"email": email, "password": password }))
        .await
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Must deserialize to LoginResponse");

    let (_, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email_value)
        .await
        .unwrap_or_else(|_| panic!("2FA code for {email} must be in store"));

    let test_cases = [
        json!({"email": "missing@example.com", "loginAttemptId": login_attempt_id, "2FACode": two_fa_code.as_ref()}),
        json!({"email": email, "loginAttemptId": login_attempt_id, "2FACode":  two_fa_code.as_ref().chars().rev().collect::<String>()}),
        json!({"email": email, "loginAttemptId": Uuid::new_v4(), "2FACode": two_fa_code.as_ref()}),
    ];

    for test_case in test_cases {
        let app = TestApp::new().await;
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status(), 401);
    }
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail.

    let app = TestApp::new().await;
    let email = TestApp::get_random_email();
    let email_value = Email::parse(&email).unwrap();
    let password = "password".to_owned();

    app.post_signup(&json!({
        "email": email,
        "password": password,
        "requires2FA": true
    }))
    .await
    .json::<SignupResponse>()
    .await
    .expect("Must deserialize to SignupResponse");

    app.post_login(&json!({"email": email, "password": password }))
        .await
        .json::<LoginResponse>()
        .await
        .expect("Must deserialize to LoginResponse");

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email_value)
        .await
        .unwrap_or_else(|_| panic!("2FA code for {email} must be in store"));

    // second login to invalidate the 1st login 2FA code
    app.post_login(&json!({"email": email, "password": password }))
        .await
        .json::<LoginResponse>()
        .await
        .expect("Must deserialize to LoginResponse");

    let app = TestApp::new().await;
    let response = app
        .post_verify_2fa(&json!({
            "email": email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode":two_fa_code.as_ref()
        }))
        .await;
    assert_eq!(response.status(), 401);
}
