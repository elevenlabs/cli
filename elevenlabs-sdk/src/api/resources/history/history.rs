use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct HistoryClient {
    pub http_client: HttpClient,
}

impl HistoryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of your generated audio (e.g. text to speech, speech to speech, Studio, dubbing). Music and SFX generations are not included and cannot currently be retrieved via the API.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many history items to return at maximum. Can not exceed 1000, defaults to 100.
    /// * `start_after_history_item_id` - After which ID to start fetching, use this parameter to paginate across a large collection of history items. In case this parameter is not provided history items will be fetched starting from the most recently created one ordered descending by their creation date.
    /// * `voice_id` - ID of the voice to be filtered for. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `model_id` - Search term used for filtering history items. If provided, source becomes required.
    /// * `date_before_unix` - Unix timestamp to filter history items before this date (exclusive).
    /// * `date_after_unix` - Unix timestamp to filter history items after this date (inclusive).
    /// * `sort_direction` - Sort direction for the results.
    /// * `search` - search term used for filtering
    /// * `source` - Source of the generated history item
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &HistoryListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpeechHistoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/history",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string(
                        "start_after_history_item_id",
                        request.start_after_history_item_id.clone(),
                    )
                    .string("voice_id", request.voice_id.clone())
                    .string("model_id", request.model_id.clone())
                    .int("date_before_unix", request.date_before_unix.clone())
                    .int("date_after_unix", request.date_after_unix.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .string("search", request.search.clone())
                    .serialize("source", request.source.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a history item.
    ///
    /// # Arguments
    ///
    /// * `history_item_id` - ID of the history item to be used. You can use the [Get generated items](/docs/api-reference/history/list) endpoint to retrieve a list of history items.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        history_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SpeechHistoryItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/history/{}", history_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a history item by its ID
    ///
    /// # Arguments
    ///
    /// * `history_item_id` - ID of the history item to be used. You can use the [Get generated items](/docs/api-reference/history/list) endpoint to retrieve a list of history items.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        history_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteHistoryItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/history/{}", history_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the audio of an history item.
    ///
    /// # Arguments
    ///
    /// * `history_item_id` - ID of the history item to be used. You can use the [Get generated items](/docs/api-reference/history/list) endpoint to retrieve a list of history items.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn get_audio(
        &self,
        history_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/history/{}/audio", history_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Download one or more history items. If one history item ID is provided, we will return a single audio file. If more than one history item IDs are provided, we will provide the history items packed into a .zip file.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn download(
        &self,
        request: &DownloadHistoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/history/download",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
