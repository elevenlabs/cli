use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CompositionPlanClient {
    pub http_client: HttpClient,
}

impl CompositionPlanClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a composition plan for music generation. Usage of this endpoint does not cost any credits but is subject to rate limiting depending on your tier.
    ///
    /// # Arguments
    ///
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. Zero retention mode may only be used by enterprise customers.
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
    ///         .music
    ///         .composition_plan
    ///         .create(
    ///             &BodyGenerateCompositionPlanV1MusicPlanPost {
    ///                 enable_logging: Some(true),
    ///                 prompt: "prompt".to_string(),
    ///                 music_length_ms: None,
    ///                 source_composition_plan: None,
    ///                 model_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &BodyGenerateCompositionPlanV1MusicPlanPost,
        options: Option<RequestOptions>,
    ) -> Result<CompositionPlanCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/music/plan",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("enable_logging", request.enable_logging.clone())
                    .build(),
                options,
            )
            .await
    }
}
