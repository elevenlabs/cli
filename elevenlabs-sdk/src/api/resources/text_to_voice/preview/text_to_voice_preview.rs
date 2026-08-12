use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PreviewClient {
    pub http_client: HttpClient,
}

impl PreviewClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Stream a voice preview that was created via the /v1/text-to-voice/design endpoint.
    ///
    /// # Arguments
    ///
    /// * `generated_voice_id` - The generated_voice_id to stream.
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
    ///         .text_to_voice
    ///         .preview
    ///         .stream(&"generated_voice_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn stream(
        &self,
        generated_voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/text-to-voice/{}/stream", generated_voice_id),
                None,
                None,
                options,
            )
            .await
    }
}
