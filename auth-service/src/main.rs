use std::sync::Arc;

use auth_service::app_state::AppState;
use auth_service::services::{HashmapUserStore, HashsetBannedTokenStore};
use auth_service::utils::constants::prod;
use auth_service::Application;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::default();
    let banned_token_store = HashsetBannedTokenStore::default();
    let app_state = AppState::new(
        Arc::new(RwLock::new(user_store)),
        Arc::new(RwLock::new(banned_token_store)),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
