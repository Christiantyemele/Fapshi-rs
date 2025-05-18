pub mod api;
/// Fapshi SDK for Rust
///
/// This crate provides a convenient and type-safe interface for interacting with the Fapshi payment service API.
/// It supports creating payment links, querying transaction statuses, expiring transactions, retrieving transactions by user ID,
/// initiating direct payments, searching transactions, configuring webhooks, and checking service balance.
///
/// # Features
/// - Authenticated API requests using `apiuser` and `apikey`.
/// - Support for both sandbox and production environments.
/// - Modular design with APIs for payments, transactions, webhooks, and balance.
/// - Comprehensive error handling with custom error types.
/// - Optional async support with the `async` feature, enabling asynchronous API calls for integration with async runtimes.
///
/// # Example (Synchronous)
/// ```
/// use fapshi_sdk::{FapshiClient, models::PaymentRequest, api::payment::PaymentApi};
///
/// use std::env;
/// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
/// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
/// let client = FapshiClient::new(&api_user, &api_key, true)?;
/// let request = PaymentRequest {
///     amount: 100.0,
///     currency: "USD".to_string(),
///     description: "Test payment".to_string(),
///     customer_email: Some("test@example.com".to_string()),
///     user_id: None,
/// };
/// let response = PaymentApi::create_payment(&client, &request).unwrap();
/// println!("Payment link: {}", response.payment_link);
/// ```
///
/// # Example (Asynchronous)
/// ```
/// use fapshi_sdk::{FapshiClient, models::PaymentRequest, api::payment::PaymentApi};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
/// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
/// let client = FapshiClient::new(&api_user, &api_key, true)?;
/// let request = PaymentRequest {
///     amount: 100.0,
///     currency: "USD".to_string(),
///     description: "Test payment".to_string(),
///     customer_email: Some("test@example.com".to_string()),
///     user_id: None,
/// };
/// let response = PaymentApi::create_payment(&client, &request).await.unwrap();
/// println!("Payment link: {}", response.payment_link);
/// # Ok(())
/// # }
/// ```
pub mod client;
pub mod error;
pub mod models;