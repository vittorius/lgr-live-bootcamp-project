use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[tracing::instrument(name = "Verify 2FA code", skip_all)]
pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(StatusCode, CookieJar), AuthAPIError> {
    let email = Email::parse(request.email.into()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id.into())
        .map_err(|_| AuthAPIError::InvalidCredentials)?; // Validate the login attempt ID in `request`
    let two_fa_code =
        TwoFACode::parse(request.two_fa_code).map_err(|_| AuthAPIError::InvalidCredentials)?; // Validate the 2FA code in `request`

    // TODO: reduce the lock scope here and everywhere else
    // see https://discord.com/channels/818251276378701824/1433205499775680594/1434195659958784220
    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let code_tuple = two_fa_code_store
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if !(code_tuple.0 == login_attempt_id && code_tuple.1 == two_fa_code) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    two_fa_code_store
        .remove_code(&email)
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;

    let auth_cookie = generate_auth_cookie(&email).map_err(AuthAPIError::UnexpectedError)?;
    let updated_jar = jar.add(auth_cookie);

    Ok((StatusCode::OK, updated_jar))
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}
