use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse, routing::post, serve::Serve, Router};
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(Self::signup))
            .route("/login", post(Self::login))
            .route("/verify-2fa", post(Self::verify_2fa))
            .route("/logout", post(Self::logout))
            .route("/verify-token", post(Self::verify_token));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }

    async fn signup() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }

    async fn login() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }

    async fn verify_2fa() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }

    async fn logout() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }

    async fn verify_token() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }
}
