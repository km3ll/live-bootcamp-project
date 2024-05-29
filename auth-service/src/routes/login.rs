use axum::{http::StatusCode, Json, response::IntoResponse};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, Password};
use crate::routes::SignupResponse;

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>
) -> Result<impl IntoResponse, AuthAPIError> {

    // Validations
    let email = Email::parse(request.email.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;
    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }
    let user = user_store.get_user(&email).await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let response = Json(LoginResponse {
        message: "Login successful".to_string()
    });
    Ok((StatusCode::OK, response))

}