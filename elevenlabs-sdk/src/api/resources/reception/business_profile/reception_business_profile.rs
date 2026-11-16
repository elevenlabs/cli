use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BusinessProfileClient {
    pub http_client: HttpClient,
}

impl BusinessProfileClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get the business profile the receptionists answer from, with the current time in the business's timezone.
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
    ///         .reception
    ///         .business_profile
    ///         .get(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionAgentInfoResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/reception/agent-info", None, None, options)
            .await
    }

    /// Update the business profile. Every receptionist is updated to match.
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
    ///         .reception
    ///         .business_profile
    ///         .update(
    ///             &ReceptionUpdateAgentInfoRequest {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        request: &ReceptionUpdateAgentInfoRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionAgentInfoResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/reception/agent-info",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
