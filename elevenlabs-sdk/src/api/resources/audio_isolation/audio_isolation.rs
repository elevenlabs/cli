use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AudioIsolationClient {
    pub http_client: HttpClient,
}

impl AudioIsolationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Removes background noise from audio.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn convert(
        &self,
        request: &ConvertRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_multipart_stream_request(
                Method::POST,
                "v1/audio-isolation",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Returns a list of all your audio isolation generations.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many history items to return at maximum. Defaults to 100.
    /// * `page` - Page number for search pagination (1-based). Only used when search is provided.
    /// * `search` - Optional search term used for filtering audio isolation history (title/text).
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
    ///         .audio_isolation
    ///         .list(
    ///             &AudioIsolationListQueryRequest {
    ///                 page_size: Some(1),
    ///                 page: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AudioIsolationListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAudioIsolationHistoryResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/audio-isolation/history",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .string("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a specific audio isolation history item and the associated media files.
    ///
    /// # Arguments
    ///
    /// * `history_item_id` - Identifier of the audio isolation history item.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .audio_isolation
    ///         .delete(&"history_item_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        history_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/audio-isolation/history/{}", history_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes background noise from audio.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn stream(
        &self,
        request: &StreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_multipart_stream_request(
                Method::POST,
                "v1/audio-isolation/stream",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
