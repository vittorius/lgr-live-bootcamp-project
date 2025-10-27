use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, BannedTokenStore, Email, Password, UserStore},
    utils::auth::generate_auth_cookie,
};

pub async fn login(
    State(state): State<AppState<impl UserStore, impl BannedTokenStore>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(StatusCode, CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(&request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let user = user_store
        .get_user(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    match user.requires_2fa {
        true => handle_2fa(jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar, Json<LoginResponse>), AuthAPIError> {
    Ok((
        StatusCode::PARTIAL_CONTENT,
        jar,
        LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".to_owned(),
            login_attempt_id: "123456".to_owned(),
        }).into(),
    ))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar, Json<LoginResponse>), AuthAPIError> {
    let auth_cookie = generate_auth_cookie(email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    Ok((
        StatusCode::OK,
        updated_jar,
        LoginResponse::RegularAuth.into(),
    ))
}
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
