use crate::helpers::TestApp;
use auth_service::{
    domain::Email,
    utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME},
    ErrorResponse,
};
use reqwest::Url;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();
    let auth_cookie = generate_auth_cookie(&Email::parse(&random_email).expect("Invalid email"))
        .expect("Failed to generate auth cookie");
    app.cookie_jar.add_cookie_str(
        &auth_cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();
    let auth_cookie = generate_auth_cookie(&Email::parse(&random_email).expect("Invalid email"))
        .expect("Failed to generate auth cookie");
    app.cookie_jar.add_cookie_str(
        &auth_cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    app.logout().await;
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid input".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "JWT is not valid".to_owned()
    );
}
