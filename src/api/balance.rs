use crate::{client::FapshiClient, error::FapshiError, models::ServiceBalance};

/// API for retrieving service balance.
pub struct BalanceApi;

impl BalanceApi {
    /// Retrieves the current service account balance.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    ///
    /// # Returns
    /// A `Result` containing the `ServiceBalance` with the balance and currency,
    /// or a `FapshiError` if the request fails.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::balance::BalanceApi};
    ///
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
    /// let balance = BalanceApi::get_service_balance(&client).unwrap();
    /// println!("Balance: {} {}", balance.balance, balance.currency);
    /// ```
    pub fn get_service_balance(client: &FapshiClient) -> Result<ServiceBalance, FapshiError> {
        let response = client.get("balance")?;
        let balance: ServiceBalance = serde_json::from_str(&response)?;
        Ok(balance)
    }
}