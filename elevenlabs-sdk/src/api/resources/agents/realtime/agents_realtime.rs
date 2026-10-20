use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct RealtimeClient {
    pub http_client: HttpClient,
}

impl RealtimeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create an ephemeral client secret for authenticating browser-based Realtime API sessions.
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
    ///     client
    ///         .agents
    ///         .realtime
    ///         .create_client_secret(
    ///             &Some(CreateClientSecretRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_client_secret(
        &self,
        request: &Option<CreateClientSecretRequest>,
        options: Option<RequestOptions>,
    ) -> Result<ClientSecretResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/realtime/client_secrets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
