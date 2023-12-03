use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use serde_json::json;
use sqlx::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum AppError {
    Event(EventError),
    Database(sqlx::Error),
    MissingCredentials,
    InvalidPassword,
    UserDoesNotExist,
    UserAlreadyExists,
    InvalidToken,
    InternalServerError,
    #[allow(dead_code)]
    Any(anyhow::Error),
    RequestAPI(ReqwestError),
    SerdeParse(SerdeError)
}

#[derive(derive_more::Display, Debug)]
pub enum EventError {
    InvalidId,
}

impl From<reqwest::Error> for AppError {
    fn from(value: ReqwestError) -> Self {
        AppError::RequestAPI(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: SerdeError) -> Self {
        AppError::SerdeParse(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Event(err) => match err {
                EventError::InvalidId => (StatusCode::NOT_FOUND, err.to_string()),
            },
            AppError::Database(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "Your credentials were missing or otherwise incorrect".to_string(),
            ),
            AppError::UserDoesNotExist => (
                StatusCode::UNAUTHORIZED,
                "Your account does not exist!".to_string(),
            ),
            AppError::UserAlreadyExists => (
                StatusCode::UNAUTHORIZED,
                "There is already an account with that email address in the system".to_string(),
            ),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token".to_string()),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid Password".to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something terrible happened".to_string(),
            ),
            AppError::RequestAPI(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::SerdeParse(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value)
    }
}