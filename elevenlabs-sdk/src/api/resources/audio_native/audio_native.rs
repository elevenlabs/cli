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
    ///         .audio_native
    ///         .create(
    ///             &CreateRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 name: "name".to_string(),
    ///                 image: None,
    ///                 author: None,
    ///                 title: None,
    ///                 small: None,
    ///                 text_color: None,
    ///                 background_color: None,
    ///                 sessionization: None,
    ///                 voice_id: None,
    ///                 model_id: None,
    ///                 auto_convert: None,
    ///                 apply_text_normalization: None,
    ///                 pronunciation_dictionary_locators: None,
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
    ///         .audio_native
    ///         .get_settings(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
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
    ///         .audio_native
    ///         .update(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &UpdateRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 auto_convert: None,
    ///                 auto_publish: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .audio_native
    ///         .update_content_from_url(
    ///             &BodyUpdateAudioNativeContentFromURLV1AudioNativeContentPost {
    ///                 url: "https://elevenlabs.io/blog/the_first_ai_that_can_laugh/".to_string(),
    ///                 author: None,
    ///                 title: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
