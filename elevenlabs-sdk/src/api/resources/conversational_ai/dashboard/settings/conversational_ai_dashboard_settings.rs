use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SettingsClient2 {
    pub http_client: HttpClient,
}

impl SettingsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve Convai dashboard settings for the workspace
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
    ///     client.conversational_ai.dashboard.settings.get(None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetConvAiDashboardSettingsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/settings/dashboard",
                None,
                None,
                options,
            )
            .await
    }

    /// Update Convai dashboard settings for the workspace
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
    ///         .conversational_ai
    ///         .dashboard
    ///         .settings
    ///         .update(
    ///             &PatchConvAiDashboardSettingsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        request: &PatchConvAiDashboardSettingsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConvAiDashboardSettingsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                "v1/convai/settings/dashboard",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
