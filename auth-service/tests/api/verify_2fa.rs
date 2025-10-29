use crate::helpers::TestApp;
use serde_json::json;

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
