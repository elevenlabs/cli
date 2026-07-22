use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod pronunciation_dictionaries;
pub use pronunciation_dictionaries::PronunciationDictionariesClient2;
pub mod content;
pub use content::ContentClient;
pub mod snapshots;
pub use snapshots::SnapshotsClient;
pub mod chapters;
pub use chapters::ChaptersClient;
pub struct ProjectsClient {
    pub http_client: HttpClient,
    pub pronunciation_dictionaries: PronunciationDictionariesClient2,
    pub content: ContentClient,
    pub snapshots: SnapshotsClient,
    pub chapters: ChaptersClient,
}

impl ProjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            pronunciation_dictionaries: PronunciationDictionariesClient2::new(config.clone())?,
            content: ContentClient::new(config.clone())?,
            snapshots: SnapshotsClient::new(config.clone())?,
            chapters: ChaptersClient::new(config.clone())?,
        })
    }

    /// Returns a list of your Studio projects with metadata.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetProjectsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/studio/projects", None, None, options)
            .await
    }

    /// Creates a new Studio project, it can be either initialized as blank, from a document or from a URL.
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
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddProjectResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/studio/projects",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Returns information about a specific Studio project. This endpoint returns more detailed information about a project than `GET /v1/studio`.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `share_id` - The share ID of the project
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        project_id: &str,
        request: &StudioProjectsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProjectExtendedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/studio/projects/{}", project_id),
                None,
                QueryBuilder::new()
                    .string("share_id", request.share_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the specified Studio project by setting the values of the parameters passed.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        project_id: &str,
        request: &BodyUpdateStudioProjectV1StudioProjectsProjectIdPost,
        options: Option<RequestOptions>,
    ) -> Result<EditProjectResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/studio/projects/{}", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a Studio project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteProjectResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/studio/projects/{}", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Starts conversion of a Studio project and all of its chapters.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConvertProjectResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/studio/projects/{}/convert", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a list of chapter IDs that have muted tracks in a project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the Studio project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_muted_tracks(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectMutedTracksResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/studio/projects/{}/muted-tracks", project_id),
                None,
                None,
                options,
            )
            .await
    }
}
