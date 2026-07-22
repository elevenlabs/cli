use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod subscription;
pub use subscription::SubscriptionClient;
pub struct UserClient {
    pub http_client: HttpClient,
    pub subscription: SubscriptionClient,
}

impl UserClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            subscription: SubscriptionClient::new(config.clone())?,
        })
    }

    /// Gets information about the user
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
    ///     client.user.get(None).await;
    /// }
    /// ```
    pub async fn get(&self, options: Option<RequestOptions>) -> Result<User, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/user", None, None, options)
            .await
    }
}
