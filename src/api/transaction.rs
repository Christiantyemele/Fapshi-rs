use crate::{
    client::FapshiClient,
    error::FapshiError,
    models::{PaymentTransactionResponse, TransactionStatus},
};

/// API for querying and managing transactions.
pub struct TransactionApi;

impl TransactionApi {
    /// Retrieves the status of a transaction synchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `transaction_id` - The unique ID of the transaction to query.
    ///
    /// # Returns
    /// A `Result` containing the `TransactionStatus` or a `FapshiError` if the request fails.
    #[cfg(not(feature = "async"))]
    pub fn get_status(
        client: &FapshiClient,
        transaction_id: &str,
    ) -> Result<TransactionStatus, FapshiError> {
        let endpoint = format!("payment-status/{}", transaction_id);
        let response = client.get(&endpoint)?;
        println!("{}", response);
        let status: TransactionStatus = serde_json::from_str(&response)?;
        Ok(status)
    }

    /// Retrieves the status of a transaction asynchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `transaction_id` - The unique ID of the transaction to query.
    ///
    /// # Returns
    /// A `Result` containing the `TransactionStatus` or a `FapshiError` if the request fails.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::transaction::TransactionApi};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true)?;
    /// let status = TransactionApi::get_status(&client, "trans123").await.unwrap();
    /// println!("Transaction status: {:?}", status);
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_status(
        client: &FapshiClient,
        transaction_id: &str,
    ) -> Result<TransactionStatus, FapshiError> {
        let endpoint = format!("payment-status/{}", transaction_id);
        let response = client.get(&endpoint).await?;
        println!("{}", response);
        let status: TransactionStatus = serde_json::from_str(&response)?;
        Ok(status)
    }

    /// Expires a payment transaction to prevent further payments synchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `transaction_id` - The unique ID of the transaction to expire.
    ///
    /// # Returns
    /// A `Result` indicating success or a `FapshiError` if the request fails (e.g., if already expired).
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::transaction::TransactionApi};
    ///
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
    /// TransactionApi::expire_transaction(&client, "trans123").unwrap();
    /// ```
    #[cfg(not(feature = "async"))]
    pub fn expire_transaction(
        client: &FapshiClient,
        transaction_id: &str,
    ) -> Result<(), FapshiError> {
        let endpoint = "expire-pay";
        let transaction = serde_json::to_string(&PaymentTransactionResponse {
            transaction_id: transaction_id.to_string(),
        })?;
        client.post(endpoint, &transaction)?;
        Ok(())
    }

    /// Expires a payment transaction to prevent further payments asynchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `transaction_id` - The unique ID of the transaction to expire.
    ///
    /// # Returns
    /// A `Result` indicating success or a `FapshiError` if the request fails (e.g., if already expired).
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::transaction::TransactionApi};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true)?;
    /// TransactionApi::expire_transaction(&client, "trans123").await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn expire_transaction(
        client: &FapshiClient,
        transaction_id: &str,
    ) -> Result<(), FapshiError> {
        let endpoint = "expire-pay";
        let transaction = serde_json::to_string(&PaymentTransactionResponse {
            transaction_id: transaction_id.to_string(),
        })?;
        client.post(endpoint, &transaction).await?;
        Ok(())
    }

    /// Retrieves all transactions associated with a user ID synchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `user_id` - The user ID to query transactions for.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<TransactionStatus>` with matching transactions or a `FapshiError`.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::transaction::TransactionApi};
    ///
    /// use std::env;
    /// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    /// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    /// let client = FapshiClient::new(&api_user, &api_key, true)?;
    /// let transactions = TransactionApi::get_transactions_by_user_id(&client, "user123").unwrap();
    /// for tx in transactions {
    ///     println!("Transaction ID: {}, Status: {}", tx.transaction_id, tx.status);
    /// }
    /// ```
    #[cfg(not(feature = "async"))]
    pub fn get_transactions_by_user_id(
        client: &FapshiClient,
        user_id: &str,
    ) -> Result<Vec<TransactionStatus>, FapshiError> {
        let endpoint = format!("transaction/{}", user_id);
        let response = client.get(&endpoint)?;
        let transactions: Vec<TransactionStatus> = serde_json::from_str(&response)?;
        Ok(transactions)
    }

    /// Retrieves all transactions associated with a user ID asynchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `user_id` - The user ID to query transactions for.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<TransactionStatus>` with matching transactions or a `FapshiError`.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::transaction::TransactionApi};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    /// let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    /// let client = FapshiClient::new(&api_user, &api_key, true)?;
    /// let transactions = TransactionApi::get_transactions_by_user_id(&client, "user123").await.unwrap();
    /// for tx in transactions {
    ///     println!("Transaction ID: {}, Status: {}", tx.transaction_id, tx.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn get_transactions_by_user_id(
        client: &FapshiClient,
        user_id: &str,
    ) -> Result<Vec<TransactionStatus>, FapshiError> {
        let endpoint = format!("transaction/{}", user_id);
        let response = client.get(&endpoint).await?;
        let transactions: Vec<TransactionStatus> = serde_json::from_str(&response)?;
        Ok(transactions)
    }

    // /// Searches transactions based on specified criteria (unchanged, commented out).
    // /// ...
    // pub fn search_transactions(client: &FapshiClient, query: &TransactionSearchQuery) -> Result<TransactionList, FapshiError> {
    //     ...
    // }
}