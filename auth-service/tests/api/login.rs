use auth_service::{
    domain::{Email, LoginAttemptId},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};
use serde_json::json;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let test_cases = [
        json!({
            "email": random_email,
        }),
        json!({
           "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        json!({
            "email": "",
            "password": "password123",
        }),
        json!({
            "email": "malformed",
            "password": "password123",
        }),
        json!({
            "email": "@example.com",
            "password": "password123",
        }),
        json!({
            "email": "me@",
            "password": "password123",
        }),
        json!({
            "email": TestApp::get_random_email(),
            "password": "short",
        }),
        json!({
            "email": TestApp::get_random_email(),
            "password": "",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
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

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let test_app = TestApp::new().await;

    let response = test_app
        .post_login(&json!({
            "email": "me@example.com",
            "password": "password123",
        }))
        .await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "Failed for input: {:?}",
        json!({
            "email": "me@example.com",
            "password": "password123",
        })
    );
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Incorrect credentials".to_owned()
    );
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());
    assert_eq!(
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(&random_email).expect("Must be valid email"))
            .await
            .expect("Login attempt ID must be present in store")
            .0,
        LoginAttemptId::parse(json_body.login_attempt_id).expect("Must be valid")
    );
}
