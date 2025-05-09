use crate::{client::FapshiClient, error::FapshiError, models::WebhookConfig};

/// API for configuring webhooks.
pub struct WebhookApi;

impl WebhookApi {
    /// Configures a webhook for receiving payment status updates.
    ///
    /// # Arguments
    /// * `client` - The `FapshiClient` instance for making API requests.
    /// * `config` - The `WebhookConfig` containing the webhook URL and service ID.
    ///
    /// # Returns
    /// A `Result` indicating success or a `FapshiError` if the request fails.
    pub fn configure_webhook(
        client: &FapshiClient,
        config: &WebhookConfig,
    ) -> Result<(), FapshiError> {
        let url = &config.url;
        client.post(url, "{}")?;
        Ok(())
    }
}
