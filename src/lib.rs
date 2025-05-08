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
///
/// # Example
/// ```
/// use fapshi_sdk::{FapshiClient, models::PaymentRequest, api::payment::PaymentApi};
///
/// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
/// let request = PaymentRequest {
///     amount: 100.0,
///     currency: "USD".to_string(),
///     description: "Test payment".to_string(),
///     customer_email: Some("test@example.com".to_string()),
/// };
/// let response = PaymentApi::create_payment(&client, &request).unwrap();
/// println!("Payment link: {}", response.payment_link);
/// ``
pub mod client;
pub mod api;
pub mod error;
pub mod models;
