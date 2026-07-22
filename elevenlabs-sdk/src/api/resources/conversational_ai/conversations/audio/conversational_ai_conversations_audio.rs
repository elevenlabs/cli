use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AudioClient {
    pub http_client: HttpClient,
}

impl AudioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get the audio recording of a particular conversation
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn get(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/convai/conversations/{}/audio", conversation_id),
                None,
                None,
                options,
            )
            .await
    }
}
