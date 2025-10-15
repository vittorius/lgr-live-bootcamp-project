use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User, UserStore, UserStoreError},
};

pub async fn signup(
    State(state): State<AppState<impl UserStore>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = &request.email;
    let password = &request.password;

    if email.is_empty() || !email.contains('@') || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    if let Err(UserStoreError::UserAlreadyExists) = user_store.add_user(user).await {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    let response = Json(SignupResponse {
        message: "User created successfully".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SignupResponse {
    pub message: String,
}
