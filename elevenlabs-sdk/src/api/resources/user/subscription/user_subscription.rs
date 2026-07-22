use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SubscriptionClient {
    pub http_client: HttpClient,
}

impl SubscriptionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets extended information about the users subscription
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client.user.subscription.get(None).await;
    /// }
    /// ```
    pub async fn get(&self, options: Option<RequestOptions>) -> Result<Subscription, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/user/subscription", None, None, options)
            .await
    }
}
