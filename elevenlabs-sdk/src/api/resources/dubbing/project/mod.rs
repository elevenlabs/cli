use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod language;
pub use language::LanguageClient;
pub mod transcript;
pub use transcript::TranscriptClient2;
pub struct ProjectClient {
    pub http_client: HttpClient,
    pub language: LanguageClient,
    pub transcript: TranscriptClient2,
}

impl ProjectClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            language: LanguageClient::new(config.clone())?,
            transcript: TranscriptClient2::new(config.clone())?,
        })
    }

    /// List the workspace's dubbing projects (cursor-paginated).
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from a previous response's next_cursor.
    /// * `page_size` - Number of projects per page (max 100).
    /// * `status` - Filter to projects in this status (preparing, ready, failed).
    /// * `sort_direction` - Sort by creation time (default 'DESCENDING').
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
    ///         .list(
    ///             &DubbingProjectListQueryRequest {
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
        request: &DubbingProjectListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingProjectListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/dubbing/project",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .string("status", request.status.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a dubbing project from an uploaded file or a source URL.
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
    ///         .dubbing
    ///         .project
    ///         .create(
    ///             &CreateRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 source_url: Some("https://example.com/promo.mp4".to_string()),
    ///                 reference: Some("Q3 marketing video".to_string()),
    ///                 source_language: Some("en".to_string()),
    ///                 model_id: None,
    ///                 keyterms: None,
    ///                 target_language: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingProjectResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/dubbing/project",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Full project detail, including its language target ids.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project to fetch.
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
    ///         .get(&"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingProjectResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/project/{}", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a project and its language targets.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project to delete.
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
    ///         .delete(&"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/dubbing/project/{}", project_id),
                None,
                None,
                options,
            )
            .await
    }
}
