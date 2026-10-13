use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AudioEffectsClient {
    pub http_client: HttpClient,
}

impl AudioEffectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the filter presets, reverb environments, and background noises that `audio_effects` can reference in Text to Speech, Text to Dialogue, and agent configurations.
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
    ///     client.audio_effects.get_catalog(None).await;
    /// }
    /// ```
    pub async fn get_catalog(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<AudioEffectsCatalogResponseModel, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/audio-effects/catalog", None, None, options)
            .await
    }
}
