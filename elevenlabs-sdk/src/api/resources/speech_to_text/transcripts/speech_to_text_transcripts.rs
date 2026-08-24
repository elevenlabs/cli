use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TranscriptsClient2 {
    pub http_client: HttpClient,
}

impl TranscriptsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a previously generated transcript by its ID.
    ///
    /// # Arguments
    ///
    /// * `transcription_id` - The unique ID of the transcript to retrieve
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
    ///         .speech_to_text
    ///         .transcripts
    ///         .get(&"transcription_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        transcription_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TranscriptsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/speech-to-text/transcripts/{}", transcription_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a previously generated transcript by its ID.
    ///
    /// # Arguments
    ///
    /// * `transcription_id` - The unique ID of the transcript to delete
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
    ///         .speech_to_text
    ///         .transcripts
    ///         .delete(&"transcription_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        transcription_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/speech-to-text/transcripts/{}", transcription_id),
                None,
                None,
                options,
            )
            .await
    }
}
