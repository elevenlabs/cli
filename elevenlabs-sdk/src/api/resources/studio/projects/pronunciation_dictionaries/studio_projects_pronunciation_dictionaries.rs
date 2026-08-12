use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PronunciationDictionariesClient2 {
    pub http_client: HttpClient,
}

impl PronunciationDictionariesClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a set of pronunciation dictionaries acting on a project. This will automatically mark text within this project as requiring reconverting where the new dictionary would apply or the old one no longer does.
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
    ///     client.studio.projects.pronunciation_dictionaries.create(&"21m00Tcm4TlvDq8ikWAM".to_string(), &BodyCreatePronunciationDictionariesV1StudioProjectsProjectIDPronunciationDictionariesPost {
    ///         pronunciation_dictionary_locators: vec![PronunciationDictionaryVersionLocator {
    ///             pronunciation_dictionary_id: "pronunciation_dictionary_id".to_string(),
    ///             ..Default::default()
    ///         }],
    ///         invalidate_affected_text: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        request: &BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPost,
        options: Option<RequestOptions>,
    ) -> Result<CreatePronunciationDictionaryResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/studio/projects/{}/pronunciation-dictionaries",
                    project_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
