use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TextToSpeechClient2 {
    pub http_client: HttpClient,
}

impl TextToSpeechClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the speech generations created through this API, newest first.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    /// * `page_size` - How many generations to return per page.
    /// * `status` - Only return generations with this lifecycle status.
    /// * `model_id` - Only return generations of this model.
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
    ///         .flows
    ///         .text_to_speech
    ///         .list(
    ///             &FlowsTextToSpeechListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 status: Some(TextToSpeechListRequestStatus::Pending),
    ///                 model_id: Some("model_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &FlowsTextToSpeechListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/flows/text-to-speech",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("status", request.status.clone())
                    .string("model_id", request.model_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Start a speech generation with the selected model. Charged per character via text-to-speech billing. Use this over `/v1/text-to-speech` for the asynchronous generation lifecycle or for models not offered there; for direct, synchronous speech synthesis, prefer `/v1/text-to-speech`.
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
    ///         .flows
    ///         .text_to_speech
    ///         .create(
    ///             &TextToSpeechGenerationRequest::ElevenFlashV25 {
    ///                 data: ElevenFlashV25Request {
    ///                     text: "The first move is what sets everything in motion.".to_string(),
    ///                     voice: "JBFqnCBsd6RMkjVDRZzb".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &TextToSpeechGenerationRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/flows/text-to-speech",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the status of a speech generation, and retrieve its output URL once completed.
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
    ///         .flows
    ///         .text_to_speech
    ///         .get(&"generation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        generation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/flows/text-to-speech/{}", generation_id),
                None,
                None,
                options,
            )
            .await
    }
}
