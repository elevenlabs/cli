use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod transcript;
pub use transcript::TranscriptClient3;
pub struct LanguageClient {
    pub http_client: HttpClient,
    pub transcript: TranscriptClient3,
}

impl LanguageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            transcript: TranscriptClient3::new(config.clone())?,
        })
    }

    /// List a project's language targets (cursor-paginated).
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the parent dubbing project.
    /// * `cursor` - Pagination cursor from a previous response's next_cursor.
    /// * `page_size` - Number of language targets per page (max 100).
    /// * `status` - Filter to targets in this status (queued, processing, completed, stale, failed).
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
    ///         .dubbing
    ///         .project
    ///         .language
    ///         .list(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &DubbingProjectLanguageListQueryRequest {
    ///                 page_size: Some(20),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        request: &DubbingProjectLanguageListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingLanguageListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/project/{}/language", project_id),
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .string("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Queue a language target for a project (starts once the project is ready).
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the parent dubbing project.
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
    ///         .dubbing
    ///         .project
    ///         .language
    ///         .create(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIDLanguagePost {
    ///                 target_language: "es".to_string(),
    ///                 model_id: Some("dubbing_v2".to_string()),
    ///                 voice_settings: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        request: &BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost,
        options: Option<RequestOptions>,
    ) -> Result<DubbingLanguageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/project/{}/language", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Full language-target detail.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the parent dubbing project.
    /// * `language_id` - Identifier of the language target to fetch.
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
    ///         .dubbing
    ///         .project
    ///         .language
    ///         .get(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"lang_1001kwkyxp0je6ktn4knsfrasx5s".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        language_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingLanguageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/project/{}/language/{}", project_id, language_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a language target.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the parent dubbing project.
    /// * `language_id` - Identifier of the language target to delete.
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
    ///         .dubbing
    ///         .project
    ///         .language
    ///         .delete(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"lang_1001kwkyxp0je6ktn4knsfrasx5s".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        project_id: &str,
        language_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/dubbing/project/{}/language/{}", project_id, language_id),
                None,
                None,
                options,
            )
            .await
    }
}
