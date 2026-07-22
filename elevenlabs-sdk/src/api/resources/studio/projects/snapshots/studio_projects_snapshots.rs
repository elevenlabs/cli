use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SnapshotsClient {
    pub http_client: HttpClient,
}

impl SnapshotsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a list of snapshots for a Studio project.
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
    ///         .snapshots
    ///         .list(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectSnapshotsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/studio/projects/{}/snapshots", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the project snapshot.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the Studio project.
    /// * `project_snapshot_id` - The ID of the Studio project snapshot.
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
    ///         .snapshots
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
        project_snapshot_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectSnapshotExtendedResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/studio/projects/{}/snapshots/{}",
                    project_id, project_snapshot_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Stream the audio from a Studio project snapshot.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `project_snapshot_id` - The ID of the Studio project snapshot.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///     client.studio.projects.snapshots.stream(&"project_id".to_string(), &"project_snapshot_id".to_string(), &BodyStreamStudioProjectAudioV1StudioProjectsProjectIDSnapshotsProjectSnapshotIDStreamPost {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn stream(
        &self,
        project_id: &str,
        project_snapshot_id: &str,
        request: &BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                &format!(
                    "v1/studio/projects/{}/snapshots/{}/stream",
                    project_id, project_snapshot_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a compressed archive of the Studio project's audio.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `project_snapshot_id` - The ID of the Studio project snapshot.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .snapshots
    ///         .stream_archive(
    ///             &"project_id".to_string(),
    ///             &"project_snapshot_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_archive(
        &self,
        project_id: &str,
        project_snapshot_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                &format!(
                    "v1/studio/projects/{}/snapshots/{}/archive",
                    project_id, project_snapshot_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
