use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
};

pub async fn post_verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)
        .map_err(|_| AuthAPIError::InvalidCredentials)?; // Validate the login attempt ID in `request`
    let two_fa_code =
        TwoFACode::parse(request.two_fa_code).map_err(|_| AuthAPIError::InvalidCredentials)?; // Validate the 2FA code in `request`

    let two_fa_code_store = state.two_fa_code_store.write().await;
    let code_tuple = two_fa_code_store
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if code_tuple.0 == login_attempt_id && code_tuple.1 == two_fa_code {
        Ok(StatusCode::OK.into_response())
    } else {
        Err(AuthAPIError::IncorrectCredentials)
    }
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}
