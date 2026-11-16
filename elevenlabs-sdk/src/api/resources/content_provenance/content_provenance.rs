use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ContentProvenanceClient {
    pub http_client: HttpClient,
}

impl ContentProvenanceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Check an audio file for an embedded ElevenLabs watermark. This endpoint is in beta and may change.
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
    ///         .content_provenance
    ///         .watermark(
    ///             &WatermarkRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 version: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn watermark(
        &self,
        request: &WatermarkRequest,
        options: Option<RequestOptions>,
    ) -> Result<WatermarkCheck, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/content-provenance/watermark",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
