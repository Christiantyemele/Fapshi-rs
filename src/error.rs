use thiserror::Error;
use reqwest::Error as ReqwestError;

/// Custom error type for the Fapshi SDK.
///
/// This enum encapsulates various errors that can occur when interacting with the Fapshi API,
/// including HTTP errors, header validation errors, API-specific errors, and serialization errors.
#[derive(Error, Debug)]
pub enum FapshiError {
    /// An error occurred during an HTTP request.
    #[error("HTTP request error: {0}")]
    HttpError(ReqwestError),

    /// An invalid header value was provided.
    #[error("Invalid header value: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    /// An error returned by the Fapshi API.
    #[error("API error: {0}")]
    ApiError(String),

    /// An error occurred during JSON serialization or deserialization.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl From<ReqwestError> for FapshiError {
    fn from(err: ReqwestError) -> Self {
        FapshiError::HttpError(err)
    }
}