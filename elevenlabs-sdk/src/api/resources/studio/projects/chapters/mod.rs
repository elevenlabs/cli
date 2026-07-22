use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod snapshots;
pub use snapshots::SnapshotsClient2;
pub struct ChaptersClient {
    pub http_client: HttpClient,
    pub snapshots: SnapshotsClient2,
}

impl ChaptersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            snapshots: SnapshotsClient2::new(config.clone())?,
        })
    }

    /// Returns a list of a Studio project's chapters.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the Studio project.
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
    ///         .studio
    ///         .projects
    ///         .chapters
    ///         .list(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetChaptersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/studio/projects/{}/chapters", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a new chapter either as blank or from a URL.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the Studio project.
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
    ///         .studio
    ///         .projects
    ///         .chapters
    ///         .create(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &BodyCreateChapterV1StudioProjectsProjectIDChaptersPost {
    ///                 name: "Chapter 1".to_string(),
    ///                 from_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        request: &BodyCreateChapterV1StudioProjectsProjectIdChaptersPost,
        options: Option<RequestOptions>,
    ) -> Result<AddChapterResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/studio/projects/{}/chapters", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns information about a specific chapter.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `chapter_id` - The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters.
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
    ///         .studio
    ///         .projects
    ///         .chapters
    ///         .get(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        chapter_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChapterWithContentResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/studio/projects/{}/chapters/{}", project_id, chapter_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a chapter.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `chapter_id` - The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters.
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
    ///         .studio
    ///         .projects
    ///         .chapters
    ///         .update(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &BodyUpdateChapterV1StudioProjectsProjectIDChaptersChapterIDPost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        project_id: &str,
        chapter_id: &str,
        request: &BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPost,
        options: Option<RequestOptions>,
    ) -> Result<EditChapterResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/studio/projects/{}/chapters/{}", project_id, chapter_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a chapter.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `chapter_id` - The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters.
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
    ///         .studio
    ///         .projects
    ///         .chapters
    ///         .delete(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        project_id: &str,
        chapter_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteChapterResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/studio/projects/{}/chapters/{}", project_id, chapter_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Starts conversion of a specific chapter.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `chapter_id` - The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters.
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
    ///         .studio
    ///         .projects
    ///         .chapters
    ///         .convert(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn convert(
        &self,
        project_id: &str,
        chapter_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConvertChapterResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/studio/projects/{}/chapters/{}/convert",
                    project_id, chapter_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
