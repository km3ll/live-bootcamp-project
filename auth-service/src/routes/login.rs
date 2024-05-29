use axum::{http::StatusCode, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}

pub async fn login(
    Json(request): Json<LoginRequest>
) -> impl IntoResponse {
    StatusCode::OK.into_response()
}