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
    /// use std::env;
    /// use fapshi_sdk::{FapshiClient, api::balance::BalanceApi};
    /// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    /// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    /// let client = FapshiClient::new(&api_user, &api_key, true)?;
    /// let balance = BalanceApi::get_service_balance(&client).unwrap();
    /// println!("Balance: {} {}", balance.balance, balance.currency);
    /// ```
    pub fn get_service_balance(client: &FapshiClient) -> Result<ServiceBalance, FapshiError> {
        let response = client.get("balance")?;
        let balance: ServiceBalance = serde_json::from_str(&response)?;
        Ok(balance)
    }
}
