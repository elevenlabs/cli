use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod projects;
pub use projects::ProjectsClient;
pub struct StudioClient {
    pub http_client: HttpClient,
    pub projects: ProjectsClient,
}

impl StudioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            projects: ProjectsClient::new(config.clone())?,
        })
    }

    /// Create and auto-convert a podcast project. Currently, the LLM cost is covered by us but you will still be charged for the audio generation. In the future, you will be charged for both the LLM and audio generation costs.
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
    ///         .studio
    ///         .create_podcast(
    ///             &BodyCreatePodcastV1StudioPodcastsPost {
    ///                 model_id: "eleven_multilingual_v2".to_string(),
    ///                 mode: BodyCreatePodcastV1StudioPodcastsPostMode::Conversation {
    ///                     data: PodcastConversationMode {
    ///                         conversation: PodcastConversationModeData {
    ///                             host_voice_id: "aw1NgEzBg83R7vgmiJt6".to_string(),
    ///                             guest_voice_id: "aw1NgEzBg83R7vgmiJt7".to_string(),
    ///                             ..Default::default()
    ///                         },
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 source: BodyCreatePodcastV1StudioPodcastsPostSource::PodcastTextSource(
    ///                     PodcastTextSource {
    ///                         r#type: "text".to_string(),
    ///                         text: "This is a test podcast.".to_string(),
    ///                     },
    ///                 ),
    ///                 quality_preset: None,
    ///                 duration_scale: None,
    ///                 language: None,
    ///                 intro: None,
    ///                 outro: None,
    ///                 instructions_prompt: None,
    ///                 highlights: None,
    ///                 callback_url: None,
    ///                 apply_text_normalization: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_podcast(
        &self,
        request: &BodyCreatePodcastV1StudioPodcastsPost,
        options: Option<RequestOptions>,
    ) -> Result<PodcastProjectResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/studio/podcasts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
