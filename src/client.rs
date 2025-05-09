use crate::error::FapshiError;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

/// The main client for interacting with the Fapshi API.
///
/// This struct manages authentication and HTTP requests to the Fapshi API. It supports both
/// sandbox and production environments, and all requests are authenticated using the provided
/// `apiuser` and `apikey`.
///
/// # Example
/// ```
/// use fapshi_rs::client::FapshiClient;
///
/// use std::env;
/// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
/// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
/// let client = FapshiClient::new(&api_user, &api_key, true)?;
/// ```
pub struct FapshiClient {
    client: Client,
    base_url: String,
    _api_user: String,
    _api_key: String,
}

impl FapshiClient {
    /// Creates a new `FapshiClient` instance.
    ///
    /// # Arguments
    /// * `api_user` - The API user ID obtained from the Fapshi dashboard.
    /// * `api_key` - The API key obtained from the Fapshi dashboard.
    /// * `sandbox` - If `true`, uses the sandbox environment; otherwise, uses production.
    ///
    /// # Returns
    /// A `Result` containing the `FapshiClient` or a `FapshiError` if initialization fails.
    ///
    /// # Errors
    /// Returns an error if the headers are invalid or the HTTP client cannot be built.
    pub fn new(api_user: &str, api_key: &str, sandbox: bool) -> Result<Self, FapshiError> {
        let base_url = if sandbox {
            "https://sandbox.fapshi.com".to_string()
        } else {
            "https://live.fapshi.com".to_string()
        };

        let mut headers = HeaderMap::new();
        headers.insert("apiuser", HeaderValue::from_str(api_user)?);
        headers.insert("apikey", HeaderValue::from_str(api_key)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build()?;

        Ok(FapshiClient {
            client,
            base_url,
            _api_user: api_user.to_string(),
            _api_key: api_key.to_string(),
        })
    }

    /// Sends a GET request to the specified API endpoint.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/transaction/status/123").
    ///
    /// # Returns
    /// A `Result` containing the response body as a `String` or a `FapshiError`.
    pub fn get(&self, endpoint: &str) -> Result<String, FapshiError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send()?.error_for_status()?;
        Ok(response.text()?)
    }

    /// Sends a POST request to the specified API endpoint with a JSON body.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "payment/create").
    /// * `body` - The JSON body as a string.
    ///
    /// # Returns
    /// A `Result` containing the response body as a `String` or a `FapshiError`.
    pub fn post(&self, endpoint: &str, body: &str) -> Result<String, FapshiError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self
            .client
            .post(&url)
            .body(body.to_string())
            .send()?
            .error_for_status()?;
        Ok(response.text()?)
    }
}
