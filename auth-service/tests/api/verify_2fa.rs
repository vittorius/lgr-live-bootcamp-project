use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email,
    routes::{SignupResponse, TwoFactorAuthResponse},
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};
use serde_json::json;
use test_helpers::api_test;
use uuid::Uuid;

#[api_test]
async fn should_return_200_if_correct_code() {
    let email = get_random_email();
    let email_value = Email::parse(email.clone().into()).expect("Must be valid email");

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

#[api_test]
async fn should_return_422_if_malformed_input() {
    let test_cases = [
        json!({"email": null, "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": null, "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": null}),
        json!({"loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "123456"}),
        json!({"email": "me@example.com", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "12345678-1234-1234-1234-123456789012"}),
        json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(response.status(), 422, "Failed for input: {:?}", test_case);
    }
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    let test_cases = [
        json!({"email": "invalid-email", "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "invalid-uuid", "2FACode": "123456"}),
        json!({"email": "me@example.com", "loginAttemptId": "12345678-1234-1234-1234-123456789012", "2FACode": "invalid-code"}),
        json!({"email": "", "loginAttemptId": "", "2FACode": ""}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(response.status(), 400);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[api_test]
async fn should_return_401_if_incorrect_credentials() {
    let email = get_random_email();
    let email_value = Email::parse(email.clone().into()).unwrap();
    let password = "password".to_owned();

    // not checking the response status because we explicitly deserialize the response body to SignupResponse
    app.post_signup(&json!({
        "email": email,
        "password": password,
        "requires2FA": true
    }))
    .await
    .json::<SignupResponse>()
    .await
    .expect("Must deserialize to SignupResponse");

    let response_body = app
        .post_login(&json!({"email": email, "password": password }))
        .await
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Must deserialize to LoginResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email_value)
        .await
        .unwrap_or_else(|_| panic!("2FA code for {email} must be in store"));

    assert_eq!(response_body.login_attempt_id, login_attempt_id.as_ref());

    let two_fa_code = two_fa_code.as_ref();

    let test_cases = [
        json!({"email": "missing@example.com", "loginAttemptId": login_attempt_id.as_ref(), "2FACode": two_fa_code}),
        json!({"email": email, "loginAttemptId": login_attempt_id.as_ref(), "2FACode":  two_fa_code.chars().rev().collect::<String>()}),
        json!({"email": email, "loginAttemptId": Uuid::new_v4(), "2FACode": two_fa_code}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(response.status(), 401, "Failed for input: {:?}", test_case);
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}

#[api_test]
async fn should_return_401_if_old_code() {
    let email = get_random_email();
    let email_value = Email::parse(email.clone().into()).unwrap();
    let password = "password".to_owned();

    // not checking the status code because we explicitly deserialize the response body into SignupResponse
    app.post_signup(&json!({
        "email": email,
        "password": password,
        "requires2FA": true
    }))
    .await
    .json::<SignupResponse>()
    .await
    .expect("Must deserialize to SignupResponse");

    let login_body = json!({
        "email": email,
        "password": password
    });

    // first login call
    let response_body = app
        .post_login(&login_body)
        .await
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Must deserialize to LoginResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email_value)
        .await
        .unwrap_or_else(|_| panic!("2FA code for {email} must be in store"));

    assert_eq!(response_body.login_attempt_id, login_attempt_id.as_ref());

    // second login to invalidate the 1st login 2FA code
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status(), 206);

    let response = app
        .post_verify_2fa(&json!({
            "email": email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": two_fa_code.as_ref()
        }))
        .await;

    assert_eq!(response.status(), 401);
}

#[api_test]
async fn should_return_401_if_same_code_twice() {
    // Verify twice with the same 2FA code. This should fail.

    let email = get_random_email();
    let email_value = Email::parse(email.clone().into()).unwrap();
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

    let response_body = app
        .post_login(&json!({"email": email, "password": password }))
        .await
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Must deserialize to LoginResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email_value)
        .await
        .unwrap_or_else(|_| panic!("2FA code for {email} must be in store"));
    assert_eq!(response_body.login_attempt_id, login_attempt_id.as_ref());

    let body = &json!({
        "email": email,
        "loginAttemptId": login_attempt_id.as_ref(),
        "2FACode":two_fa_code.as_ref()
    });

    let response = app.post_verify_2fa(body).await;
    assert_eq!(response.status(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_verify_2fa(body).await;
    assert_eq!(response.status(), 401);
}
