use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod preview;
pub use preview::PreviewClient;
pub struct TextToVoiceClient {
    pub http_client: HttpClient,
    pub preview: PreviewClient,
}

impl TextToVoiceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            preview: PreviewClient::new(config.clone())?,
        })
    }

    /// Create a voice from a text prompt.
    ///
    /// # Arguments
    ///
    /// * `output_format` - The output format of the generated audio.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_previews(
        &self,
        request: &VoiceDesignRequest,
        options: Option<RequestOptions>,
    ) -> Result<VoiceDesignPreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/text-to-voice/create-previews",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a voice from previously generated voice preview. This endpoint should be called after you fetched a generated_voice_id using POST /v1/text-to-voice/design or POST /v1/text-to-voice/:voice_id/remix.
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
        request: &BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePost,
        options: Option<RequestOptions>,
    ) -> Result<Voice, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/text-to-voice",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Design a voice via a prompt. This method returns a list of voice previews. Each preview has a generated_voice_id and a sample of the voice as base64 encoded mp3 audio. To create a voice use the generated_voice_id of the preferred preview with the /v1/text-to-voice endpoint.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn design(
        &self,
        request: &VoiceDesignRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<VoiceDesignPreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/text-to-voice/design",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remix an existing voice via a prompt. This method returns a list of voice previews. Each preview has a generated_voice_id and a sample of the voice as base64 encoded mp3 audio. To create a voice use the generated_voice_id of the preferred preview with the /v1/text-to-voice endpoint.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remix(
        &self,
        voice_id: &str,
        request: &VoiceRemixRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<VoiceDesignPreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/text-to-voice/{}/remix", voice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }
}
