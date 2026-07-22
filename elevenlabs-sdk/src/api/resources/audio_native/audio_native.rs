use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AudioNativeClient {
    pub http_client: HttpClient,
}

impl AudioNativeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates Audio Native enabled project, optionally starts conversion and returns project ID and embeddable HTML snippet.
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
    ) -> Result<AudioNativeCreateProjectResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/audio-native",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Get player settings for the specific project.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the Studio project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_settings(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetAudioNativeProjectSettingsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/audio-native/{}/settings", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates content for the specific AudioNative Project.
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
        request: &UpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioNativeEditContentResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/audio-native/{}/content", project_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Finds an AudioNative project matching the provided URL, extracts content from the URL, updates the project content, and queues it for conversion and auto-publishing.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_content_from_url(
        &self,
        request: &BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPost,
        options: Option<RequestOptions>,
    ) -> Result<AudioNativeEditContentResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/audio-native/content",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
