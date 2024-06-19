use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use redis::{Client, RedisResult};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::error::Error;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use utils::tracing::{
    make_span_with_request_id,
    on_request,
    on_response,
};

use crate::app_state::AppState;
use crate::routes::*;


pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field, so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {

        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://159.65.232.242:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                .make_span_with(make_span_with_request_id)
                .on_request(on_request)
                .on_response(on_response),
            );
        
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server: Serve<Router, Router> = axum::serve(listener, router);

        // Create a new Application instance and return it
        let app: Application = Application {
            server,
            address,
        };

        Ok(app)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }

}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {

        let (status, error_message) = match self {
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::UnexpectedError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
        };

        let body = Json(ErrorResponse { 
            error: error_message.to_string() 
        });
        (status, body).into_response()

    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}