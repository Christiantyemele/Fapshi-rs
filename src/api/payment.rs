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
    /// use fapshi_sdk::{FapshiClient, models::PaymentRequest, api::payment::PaymentApi};
    ///
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
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
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
    /// let request = DirectPaymentRequest {
    ///     amount: 50.0,
    ///     currency: "USD".to_string(),
    ///     phone_number: "1234567890".to_string(),
    ///     description: "Direct payment test".to_string(),
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
