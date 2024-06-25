use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Missing token")]
    MissingToken,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
    #[error("User already exists")]
    UserAlreadyExists,
}