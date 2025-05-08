use crate::{client::FapshiClient, error::FapshiError, models::{TransactionStatus, TransactionList}};

/// API for querying and managing transactions.
pub struct TransactionApi;

impl TransactionApi {
    /// Retrieves the status of a transaction.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `transaction_id` - The unique ID of the transaction to query.
    ///
    /// # Returns
    /// A `Result` containing the `TransactionStatus` or a `FapshiError` if the request fails.
    pub fn get_status(client: &FapshiClient, transaction_id: &str) -> Result<TransactionStatus, FapshiError> {
        let endpoint = format!("payment-status/:{}", transaction_id);
        let response = client.get(&endpoint)?;
        let status: TransactionStatus = serde_json::from_str(&response)?;
        Ok(status)
    }

    /// Expires a payment transaction to prevent further payments.
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
    pub fn expire_transaction(client: &FapshiClient, transaction_id: &str) -> Result<(), FapshiError> {
        let endpoint = "expire-pay";
        client.post(&endpoint, transaction_id)?;
        Ok(())
    }

    /// Retrieves all transactions associated with a user ID.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `user_id` - The user ID to query transactions for.
    ///
    /// # Returns
    /// A `Result` containing a `TransactionList` with matching transactions or a `FapshiError`.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::transaction::TransactionApi};
    ///
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
    /// let transactions = TransactionApi::get_transactions_by_user_id(&client, "user123").unwrap();
    /// for tx in transactions.transactions {
    ///     println!("Transaction ID: {}, Status: {}", tx.transaction_id, tx.status);
    /// }
    /// ```
    pub fn get_transactions_by_user_id(client: &FapshiClient, user_id: &str) -> Result<TransactionList, FapshiError> {
        let endpoint = format!("transaction/:{}", user_id);
        let response = client.get(&endpoint)?;
        let transactions: TransactionList = serde_json::from_str(&response)?;
        Ok(transactions)
    }

    // /// Searches transactions based on specified criteria.
    // ///
    // /// # Arguments
    // /// * `client` - The `FapshiClient` instance for making API requests.
    // /// * `query` - The `TransactionSearchQuery` with search parameters.
    // ///
    // /// # Returns
    // /// A `Result` containing a `TransactionList` with matching transactions or a `FapshiError`.
    // ///
    // /// # Example
    // /// ```
    // /// use fapshi_sdk::{FapshiClient, models::TransactionSearchQuery, api::transaction::TransactionApi};
    // ///
    // /// let client = FapshiClient::new("your_api_user", "your_api_key", true).unwrap();
    // /// let query = TransactionSearchQuery {
    // ///     status: Some("successful".to_string()),
    // ///     limit: Some(10),
    // ///     ..Default::default()
    // /// };
    // /// let transactions = TransactionApi::search_transactions(&client, &query).unwrap();
    // /// for tx in transactions.transactions {
    // ///     println!("Transaction ID: {}, Status: {}", tx.transaction_id, tx.status);
    // /// }
    // /// ```
    // pub fn search_transactions(client: &FapshiClient, query: &TransactionSearchQuery) -> Result<TransactionList, FapshiError> {
    //     let mut url = "/search".to_string();
    //     url.path_segments_mut()
    //         .map_err(|_| FapshiError::ApiError("Invalid base URL".to_string()))?
    //         .extend(&["transaction", "search"]);
        
    //     let mut pairs = url.query_pairs_mut();
    //     if let Some(status) = &query.status {
    //         pairs.append_pair("status", status);
    //     }
    //     if let Some(medium) = &query.medium {
    //         pairs.append_pair("medium", medium);
    //     }
    //     if let Some(name) = &query.name {
    //         pairs.append_pair("name", name);
    //     }
    //     if let Some(start) = &query.start {
    //         pairs.append_pair("start", start);
    //     }
    //     if let Some(end) = &query.end {
    //         pairs.append_pair("end", end);
    //     }
    //     if let Some(amt) = &query.amt {
    //         pairs.append_pair("amt", &amt.to_string());
    //     }
    //     if let Some(limit) = &query.limit {
    //         pairs.append_pair("limit", &limit.to_string());
    //     }
    //     drop(pairs);

    //     let response = client.get(url.as_str().trim_start_matches(&client.base_url))?;
    //     let transactions: TransactionList = serde_json::from_str(&response)?;
    //     Ok(transactions)
    // }
}