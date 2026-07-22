use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod project;
pub use project::ProjectClient;
pub mod resource;
pub use resource::ResourceClient;
pub mod audio;
pub use audio::AudioClient2;
pub mod transcript;
pub use transcript::TranscriptClient;
pub mod transcripts;
pub use transcripts::TranscriptsClient;
pub struct DubbingClient {
    pub http_client: HttpClient,
    pub project: ProjectClient,
    pub resource: ResourceClient,
    pub audio: AudioClient2,
    pub transcript: TranscriptClient,
    pub transcripts: TranscriptsClient,
}

impl DubbingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            project: ProjectClient::new(config.clone())?,
            resource: ResourceClient::new(config.clone())?,
            audio: AudioClient2::new(config.clone())?,
            transcript: TranscriptClient::new(config.clone())?,
            transcripts: TranscriptsClient::new(config.clone())?,
        })
    }

    /// List the dubs you have access to.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `page_size` - How many dubs to return at maximum. Can not exceed 200, defaults to 100.
    /// * `dubbing_status` - What state the dub is currently in.
    /// * `dubbing_statuses` - Filter by dubbing status.
    /// * `dubbing_models` - Filter by dubbing model generation.
    /// * `target_language_codes` - Filter by target language code.
    /// * `creation_sources` - Filter by dubbing creation source.
    /// * `filter_by_creator` - Filters who created the resources being listed, whether it was the user running the request or someone else that shared the resource with them.
    /// * `order_by` - The field to use for ordering results from this query.
    /// * `order_direction` - The order direction to use for results from this query.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &DubbingListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingMetadataPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/dubbing",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("dubbing_status", request.dubbing_status.clone())
                    .serialize_array("dubbing_statuses", request.dubbing_statuses.clone())
                    .serialize_array("dubbing_models", request.dubbing_models.clone())
                    .string_array(
                        "target_language_codes",
                        request.target_language_codes.clone(),
                    )
                    .serialize_array("creation_sources", request.creation_sources.clone())
                    .serialize("filter_by_creator", request.filter_by_creator.clone())
                    .serialize("order_by", request.order_by.clone())
                    .serialize("order_direction", request.order_direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Dubs a provided audio or video file into given language.
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
    ) -> Result<DoDubbingResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/dubbing",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Returns metadata about a dubbing project, including whether it's still in progress or not
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        dubbing_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingMetadataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/{}", dubbing_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a dubbing project.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        dubbing_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDubbingResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/dubbing/{}", dubbing_id),
                None,
                None,
                options,
            )
            .await
    }
}
