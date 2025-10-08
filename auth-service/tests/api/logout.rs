use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_200() {
    let app = TestApp::new().await;

    let response = app.logout("dummy_jwt_token").await;

    assert_eq!(response.status().as_u16(), 200);
}
