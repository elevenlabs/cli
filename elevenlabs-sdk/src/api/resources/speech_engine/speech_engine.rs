use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SpeechEngineClient {
    pub http_client: HttpClient,
}

impl SpeechEngineClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of Speech Engine resources.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many Speech Engines to return at maximum. Can not exceed 100, defaults to 30.
    /// * `search` - Search term to filter Speech Engines by name
    /// * `sort_direction` - The direction to sort the results
    /// * `sort_by` - The field to sort the results by
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
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
    ///         .speech_engine
    ///         .list(
    ///             &SpeechEngineListQueryRequest {
    ///                 page_size: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 sort_direction: Some(SortDirection::Asc),
    ///                 sort_by: Some(AgentSortBy::Name),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SpeechEngineListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSpeechEnginesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/speech-engine",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new Speech Engine resource
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
    ///         .speech_engine
    ///         .create(
    ///             &CreateSpeechEngineRequest {
    ///                 speech_engine: SpeechEngineConfig {
    ///                     ws_url: "ws_url".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 name: None,
    ///                 asr: None,
    ///                 tts: None,
    ///                 turn: None,
    ///                 vad: None,
    ///                 conversation: None,
    ///                 privacy: None,
    ///                 call_limits: None,
    ///                 language: None,
    ///                 tags: None,
    ///                 overrides: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSpeechEngineRequest,
        options: Option<RequestOptions>,
    ) -> Result<SpeechEngineResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/speech-engine",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a Speech Engine resource
    ///
    /// # Arguments
    ///
    /// * `speech_engine_id` - The speech engine ID (accepts seng_ or agent_ prefix)
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
    ///         .speech_engine
    ///         .get(&"seng_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        speech_engine_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SpeechEngineResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/speech-engine/{}", speech_engine_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a Speech Engine resource
    ///
    /// # Arguments
    ///
    /// * `speech_engine_id` - The speech engine ID (accepts seng_ or agent_ prefix)
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
    ///         .speech_engine
    ///         .delete(&"seng_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        speech_engine_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/speech-engine/{}", speech_engine_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a Speech Engine resource (partial update)
    ///
    /// # Arguments
    ///
    /// * `speech_engine_id` - The speech engine ID (accepts seng_ or agent_ prefix)
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
    ///         .speech_engine
    ///         .update(
    ///             &"seng_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &UpdateSpeechEngineRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        speech_engine_id: &str,
        request: &UpdateSpeechEngineRequest,
        options: Option<RequestOptions>,
    ) -> Result<SpeechEngineResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/speech-engine/{}", speech_engine_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
