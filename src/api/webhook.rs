use crate::{client::FapshiClient, error::FapshiError, models::WebhookConfig};

/// API for configuring webhooks.
pub struct WebhookApi;

impl WebhookApi {
    /// Configures a webhook for receiving payment status updates synchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `config` - The `WebhookConfig` containing the webhook URL and service ID.
    ///
    /// # Returns
    /// A `Result` indicating success or a `FapshiError` if the request fails.
    #[cfg(not(feature = "async"))]
    pub fn configure_webhook(
        client: &FapshiClient,
        config: &WebhookConfig,
    ) -> Result<(), FapshiError> {
        let url = &config.url;
        client.post(url, "{}")?;
        Ok(())
    }

    /// Configures a webhook for receiving payment status updates asynchronously.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `config` - The `WebhookConfig` containing the webhook URL and service ID.
    ///
    /// # Returns
    /// A `Result` indicating success or a `FapshiError` if the request fails.
    ///
    /// # Example
    /// ```
    /// use fapshi_sdk::{FapshiClient, api::webhook::WebhookApi, models::WebhookConfig};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FapshiClient::new("your_api_user", "your_api_key", true)?;
    /// let config = WebhookConfig {
    ///     url: "https://yourapp.com/webhook".to_string(),
    ///     service_id: "service123".to_string(),
    /// };
    /// WebhookApi::configure_webhook(&client, &config).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    pub async fn configure_webhook(
        client: &FapshiClient,
        config: &WebhookConfig,
    ) -> Result<(), FapshiError> {
        let url = &config.url;
        client.post(url, "{}").await?;
        Ok(())
    }
}