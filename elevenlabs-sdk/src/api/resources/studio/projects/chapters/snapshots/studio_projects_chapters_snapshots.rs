use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SnapshotsClient2 {
    pub http_client: HttpClient,
}

impl SnapshotsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets information about all the snapshots of a chapter. Each snapshot can be downloaded as audio. Whenever a chapter is converted a snapshot will automatically be created.
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
    pub async fn list(
        &self,
        project_id: &str,
        chapter_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChapterSnapshotsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/studio/projects/{}/chapters/{}/snapshots",
                    project_id, chapter_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the chapter snapshot.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the Studio project.
    /// * `chapter_id` - The ID of the chapter.
    /// * `chapter_snapshot_id` - The ID of the chapter snapshot.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        project_id: &str,
        chapter_id: &str,
        chapter_snapshot_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChapterSnapshotExtendedResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/studio/projects/{}/chapters/{}/snapshots/{}",
                    project_id, chapter_id, chapter_snapshot_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Stream the audio from a chapter snapshot. Use `GET /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots` to return the snapshots of a chapter.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `chapter_id` - The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters.
    /// * `chapter_snapshot_id` - The ID of the chapter snapshot to be used. You can use the [List project chapter snapshots](/docs/api-reference/studio/get-snapshots) endpoint to list all the available snapshots.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn stream(
        &self,
        project_id: &str,
        chapter_id: &str,
        chapter_snapshot_id: &str,
        request: &BodyStreamChapterAudioV1StudioProjectsProjectIdChaptersChapterIdSnapshotsChapterSnapshotIdStreamPost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                &format!(
                    "v1/studio/projects/{}/chapters/{}/snapshots/{}/stream",
                    project_id, chapter_id, chapter_snapshot_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
