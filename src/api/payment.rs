use crate::{
    client::FapshiClient,
    error::FapshiError,
    models::{DirectPaymentRequest, DirectPaymentResponse, PaymentRequest, PaymentResponse},
};

/// API for managing payment links and direct payments.
pub struct PaymentApi;

impl PaymentApi {
    /// Creates a new payment link.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `request` - The `PaymentRequest` containing payment details.
    ///
    /// # Returns
    /// A `Result` containing the `PaymentResponse` with the payment link and transaction ID,
    /// or a `FapshiError` if the request fails.
    ///
    /// # Example
    /// ```
    /// use std::env;
    /// use fapshi_sdk::{FapshiClient, models::PaymentRequest, api::payment::PaymentApi};
    ///
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
    pub fn create_payment(
        client: &FapshiClient,
        request: &PaymentRequest,
    ) -> Result<PaymentResponse, FapshiError> {
        let body = serde_json::to_string(request)?;
        println!("{}", body);
        let response = client.post("initiate-pay", &body).unwrap();
        let payment_response: PaymentResponse = serde_json::from_str(&response).unwrap();
        Ok(payment_response)
    }

    /// Initiates a direct payment request to a user's mobile device.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `request` - The `DirectPaymentRequest` containing payment details and phone number.
    ///
    /// # Returns
    /// A `Result` containing the `DirectPaymentResponse` with the transaction ID and status,
    /// or a `FapshiError` if the request fails.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, models::DirectPaymentRequest, api::payment::PaymentApi};
    ///
    /// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    /// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    /// let client = FapshiClient::new(&api_user, &api_key, true)?;
    /// let request = DirectPaymentRequest {
    ///     amount: 500.0,
    ///     phone: "654988322".to_string(),
    ///     medium: Some("mobile money".to_string()),
    ///     name: Some("Wilfried".to_string()),
    ///     email: Some("yemelechristian2@gmail.com".to_string()),
    ///     user_id: Some("abcdef12345".to_string()),
    ///     external_id: Some("order123".to_string()),
    ///     message: Some("Direct payment test".to_string()),
    /// };
    /// let response = PaymentApi::initiate_direct_payment(&client, &request).unwrap();
    /// println!("Transaction ID: {}", response.transaction_id);
    /// ```
    pub fn initiate_direct_payment(
        client: &FapshiClient,
        request: &DirectPaymentRequest,
    ) -> Result<DirectPaymentResponse, FapshiError> {
        let body = serde_json::to_string(request)?;
        let response = client.post("direct-pay", &body)?;
        let direct_response: DirectPaymentResponse = serde_json::from_str(&response)?;
        Ok(direct_response)
    }
}
