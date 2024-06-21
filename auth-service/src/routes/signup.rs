use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, User, Password}
};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupRequest{
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> Result<impl IntoResponse, AuthAPIError> {

    // Validations
    let email = Email::parse(request.email.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut user_store = state.user_store.write().await;
    let user = User::new(email, password, request.requires_2fa);

    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if let Err(e) = user_store.add_user(user).await {
        return Err(AuthAPIError::UnexpectedError(e.into()));
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string()
    });
    Ok((StatusCode::CREATED, response))

}