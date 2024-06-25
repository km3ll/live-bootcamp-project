use axum::{
    extract::State,
    http::StatusCode, Json,
    response::IntoResponse
};
use axum_extra::extract::CookieJar;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: Secret<String>,
    pub password: Secret<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

#[tracing::instrument(name = "Login", skip_all)]
pub async fn login(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
    Json(request): Json<LoginRequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {

    // Validations
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (cookie_jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (cookie_jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (cookie_jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (cookie_jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // handle request based on user's 2FA configuration
    match user.requires_2fa {
        true  => handle_2fa(&user.email, &state, cookie_jar).await,
        false => handle_no_2fa(&user.email, cookie_jar).await,
    }

}

#[tracing::instrument(name = "Handle 2FA flow", skip_all)]
async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar
) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if let Err(e) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())))
    }

    if let Err(e) = state
        .email_client
        .send_email(email, "2FA Code", two_fa_code.as_ref().expose_secret())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e)));
    }


    let auth_response = TwoFactorAuthResponse {
        message: String::from("2FA required"),
        login_attempt_id: login_attempt_id.as_ref().expose_secret().to_owned(),
    };
    let response = Json(LoginResponse::TwoFactorAuth(auth_response));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
    
}

#[tracing::instrument(name = "Handle non-2FA flow", skip_all)]
async fn handle_no_2fa(email: &Email, jar: CookieJar) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return(jar, Err(AuthAPIError::UnexpectedError(e))),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))

}