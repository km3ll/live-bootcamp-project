use axum::{http::StatusCode, response::IntoResponse, serve::Serve, Router, routing::method_routing::*};
use tower_http::services::ServeDir;
use std::error::Error;

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
            //1. Routes
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token));
        

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
        println!("listening on {}", &self.address);
        self.server.await
    }

}

// 2.Route Handlers
pub async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}