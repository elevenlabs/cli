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

    /// List the dubbing projects in your workspace that you can access, newest first, cursor-paginated. Listed projects carry no `language_ids`; fetch a project, or list its language targets, to see them.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pass the `next_cursor` from a previous response to fetch the page after it. Omit for the first page.
    /// * `page_size` - Number of projects per page. Clamped to between 1 and 100 rather than rejected, so a larger value returns a full page.
    /// * `status` - Filter to projects in this status: `queued`, `preparing`, `ready`, or `failed`. Omit to return every status.
    /// * `sort_direction` - Sort by creation time; newest first by default.
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

    /// Create a dubbing project from an uploaded file (`file`) or a source URL (`source_url`).
    ///
    /// Returns as soon as the project record exists, before the source has been fetched: the project starts `queued` and reaches `ready` once its source has been transcribed. Creating a project does not dub anything — add a language target to it for each language you want, or pass `target_language` to queue the first one here.
    ///
    /// Preparation can take minutes on a long source, so we recommend passing `webhook_ids` to be notified when the project turns `ready` or `failed`, rather than polling for it.
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
    ///                 transcript: b"test file content".to_vec(),
    ///                 source_url: Some("https://example.com/promo.mp4".to_string()),
    ///                 reference: Some("Q3 marketing video".to_string()),
    ///                 source_language: Some("en".to_string()),
    ///                 model_id: None,
    ///                 keyterms: None,
    ///                 webhook_ids: None,
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

    /// Full project detail, including the IDs of every language target under it. To follow a project to `ready`, we recommend a `webhook_ids` subscription rather than polling this endpoint.
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

    /// Delete a project, every language target under it, and their stored media and outputs. This cannot be undone, and a dub already running is still billed.
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
