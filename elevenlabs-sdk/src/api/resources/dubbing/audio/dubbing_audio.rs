use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AudioClient2 {
    pub http_client: HttpClient,
}

impl AudioClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns dub as a streamed MP3 or MP4 file. If this dub has been edited using Dubbing Studio you need to use the resource render endpoint as this endpoint only returns the original automatic dub result.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `language_code` - ID of the language.
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
    ///         .dubbing
    ///         .audio
    ///         .get(
    ///             &"dubbing_id".to_string(),
    ///             &"language_code".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        dubbing_id: &str,
        language_code: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/dubbing/{}/audio/{}", dubbing_id, language_code),
                None,
                None,
                options,
            )
            .await
    }
}
