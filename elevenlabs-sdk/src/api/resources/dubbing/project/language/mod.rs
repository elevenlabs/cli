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

    /// List a project's language targets, cursor-paginated, each with signed output URLs once it has produced an output.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the parent dubbing project.
    /// * `cursor` - Pass the `next_cursor` from a previous response to fetch the page after it. Omit for the first page.
    /// * `page_size` - Number of language targets per page. Clamped to between 1 and 100 rather than rejected, so a larger value returns a full page.
    /// * `status` - Filter to targets in this status: `queued`, `processing`, `completed`, `stale`, or `failed`. Omit to return every status.
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

    /// Add a language to dub a project into, and queue the dub.
    ///
    /// This is the call that produces dubbed audio, and it is billed per generation. The target is created `queued` and starts as soon as the project is `ready`, so it can be added at any point after the project is created. It inherits the project's dubbing model and cannot pick another.
    ///
    /// A project created with `webhook_ids` sends a `dubbing_language_completed` event carrying the output download URLs, so we recommend subscribing rather than polling this target to completion.
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
    ///                 voice_settings: None,
    ///                 translations: None,
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

    /// Full language-target detail. Once the target reports `completed`, `outputs` carries the signed download URLs. To learn when that happens, we recommend the project's `webhook_ids` subscription rather than polling this endpoint; fetch here when a delivered URL has expired, or to reconcile after an edit.
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

    /// Delete a language target and its outputs, leaving the project and its other languages intact. This cannot be undone, and a dub already running is still billed.
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
