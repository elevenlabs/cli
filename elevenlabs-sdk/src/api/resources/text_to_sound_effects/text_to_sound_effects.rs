use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TextToSoundEffectsClient {
    pub http_client: HttpClient,
}

impl TextToSoundEffectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Turn text into sound effects for your videos, voice-overs or video games using the most advanced sound effects models in the world.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn convert(
        &self,
        request: &CreateSoundEffectRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/sound-generation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }
}
