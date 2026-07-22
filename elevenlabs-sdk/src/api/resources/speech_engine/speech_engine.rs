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
