use std::sync::Arc;

use auth_service::app_state::AppState;
use auth_service::domain::Email;
use auth_service::services::data_stores::{
    PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore,
};
use auth_service::services::postmark_email_client::PostmarkEmailClient;
use auth_service::services::resend_email_client::ResendEmailClient;
use auth_service::utils::constants::{
    prod, DATABASE_URL, POSTMARK_AUTH_TOKEN, REDIS_HOST_NAME, RESEND_AUTH_TOKEN,
};
use auth_service::utils::tracing::init_tracing;
use auth_service::{get_postgres_pool, get_redis_client, Application};
use reqwest::Client;
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");

    let pg_pool = configure_postgresql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let email_client = Arc::new(configure_resend_email_client());

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

#[allow(dead_code)]
fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(prod::email_client::SENDER.to_owned().into()).unwrap(),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}

fn configure_resend_email_client() -> ResendEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    ResendEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(prod::email_client::SENDER.to_owned().into()).unwrap(),
        RESEND_AUTH_TOKEN.to_owned(),
        http_client,
    )
}
