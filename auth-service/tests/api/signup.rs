use auth_service::{routes::SignupResponse, ErrorResponse};
use serde_json::json;
use test_helpers::api_test;

use crate::helpers::{get_random_email, TestApp};

#[api_test]
async fn should_return_422_if_malformed_input() {
    let random_email = get_random_email();

    let test_cases = [
        json!({
            "password": "password123",
            "requires2FA": true
        }),
        json!({
            "email": random_email,
            "requires2FA": true
        }),
        json!({
            "email": random_email,
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[api_test]
async fn should_return_201_if_valid_input() {
    let response = app
        .post_signup(&json!({
            "email": get_random_email(),
            "password": "password123",
            "requires2FA": false
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully".to_string(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    let test_cases = [
        json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        json!({
            "email": "malformed",
            "password": "password123",
            "requires2FA": true
        }),
        json!({
            "email": get_random_email(),
            "password": "short",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

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

#[api_test]
async fn should_return_409_if_email_already_exists() {
    let email = get_random_email();
    let password = "password123";

    let response = app
        .post_signup(&json!({
            "email": email,
            "password": password,
            "requires2FA": false
        }))
        .await;

    assert_eq!(response.status().as_u16(), 201);

    let response = app
        .post_signup(&json!({
            "email": email,
            "password": password,
            "requires2FA": false
        }))
        .await;

    eprintln!("Failed Response: {:?}", response);
    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
