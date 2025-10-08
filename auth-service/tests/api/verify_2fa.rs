use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_returns_200() {
    let app = TestApp::new().await;

    let response = app
        .verify_2fa("test@example.com", "attempt123", "123456")
        .await;

    assert_eq!(response.status().as_u16(), 200);
}
