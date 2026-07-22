use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LanguageClient2 {
    pub http_client: HttpClient,
}

impl LanguageClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Adds the given ElevenLab Turbo V2/V2.5 language code to the resource. Does not automatically generate transcripts/translations/audio.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
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
    ///         .dubbing
    ///         .resource
    ///         .language
    ///         .add(
    ///             &"dubbing_id".to_string(),
    ///             &BodyAddALanguageToTheResourceV1DubbingResourceDubbingIDLanguagePost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add(
        &self,
        dubbing_id: &str,
        request: &BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePost,
        options: Option<RequestOptions>,
    ) -> Result<LanguageAddedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/language", dubbing_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
