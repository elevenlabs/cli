use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AudioClient5 {
    pub http_client: HttpClient,
}

impl AudioClient5 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the audio corresponding to a sample attached to a voice.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `sample_id` - ID of the sample to be used. You can use the [Get voices](/docs/api-reference/voices/get) endpoint list all the available samples for a voice.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .voices
    ///         .samples
    ///         .audio
    ///         .get(&"voice_id".to_string(), &"sample_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        voice_id: &str,
        sample_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/voices/{}/samples/{}/audio", voice_id, sample_id),
                None,
                None,
                options,
            )
            .await
    }
}
