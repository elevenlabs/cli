use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod transcripts;
pub use transcripts::TranscriptsClient2;
pub struct SpeechToTextClient {
    pub http_client: HttpClient,
    pub transcripts: TranscriptsClient2,
}

impl SpeechToTextClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            transcripts: TranscriptsClient2::new(config.clone())?,
        })
    }

    /// Transcribe an audio or video file. If webhook is set to true, the request will be processed asynchronously and results sent to configured webhooks. When use_multi_channel is true and the provided audio has multiple channels, a 'transcripts' object with separate transcripts for each channel is returned; set multichannel_output_style='combined' to instead receive a single transcript with all channels merged and sorted by time. Otherwise, returns a single transcript. The optional webhook_metadata parameter allows you to attach custom data that will be included in webhook responses for request correlation and tracking.
    ///
    /// # Arguments
    ///
    /// * `token` - A single-use authentication token created via POST /v1/single-use-token/batch_scribe. This token can only be used once and expires after 15 minutes. Alternative to API key or bearer token authentication for frontend clients.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean log and transcript storage features are unavailable for this request. Zero retention mode may only be used by enterprise customers.
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
    ///         .speech_to_text
    ///         .convert(
    ///             &ConvertRequest {
    ///                 token: Some("token".to_string()),
    ///                 enable_logging: Some(true),
    ///                 file: b"test file content".to_vec(),
    ///                 model_id: SpeechToTextConvertRequestModelID::ScribeV2,
    ///                 language_code: None,
    ///                 tag_audio_events: None,
    ///                 num_speakers: None,
    ///                 timestamps_granularity: None,
    ///                 diarize: None,
    ///                 diarization_threshold: None,
    ///                 additional_formats: None,
    ///                 file_format: None,
    ///                 cloud_storage_url: None,
    ///                 source_url: None,
    ///                 webhook: None,
    ///                 webhook_id: None,
    ///                 temperature: None,
    ///                 seed: None,
    ///                 use_multi_channel: None,
    ///                 multichannel_output_style: None,
    ///                 webhook_metadata: None,
    ///                 entity_detection: None,
    ///                 no_verbatim: None,
    ///                 use_speaker_library: None,
    ///                 detect_speaker_roles: None,
    ///                 entity_redaction: None,
    ///                 entity_redaction_mode: None,
    ///                 keyterms: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn convert(
        &self,
        request: &ConvertRequest3,
        options: Option<RequestOptions>,
    ) -> Result<SpeechToTextConvertResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/speech-to-text",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("token", request.token.clone())
                    .bool("enable_logging", request.enable_logging.clone())
                    .build(),
                options,
            )
            .await
    }
}
