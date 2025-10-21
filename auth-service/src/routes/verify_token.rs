use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::{AuthAPIError, BannedTokenStore, UserStore}, utils::auth::validate_token};

pub async fn verify_token(
    State(state): State<AppState<impl UserStore, impl BannedTokenStore>>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    validate_token(&request.token, &*state.banned_token_store.read().await)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
