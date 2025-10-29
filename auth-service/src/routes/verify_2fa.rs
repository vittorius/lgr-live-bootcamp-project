use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::{Email, LoginAttemptId, TwoFACode};

pub async fn post_verify_2fa(Json(request): Json<Verify2FARequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}
