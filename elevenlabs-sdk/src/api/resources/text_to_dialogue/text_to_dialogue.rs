use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TextToDialogueClient {
    pub http_client: HttpClient,
}

impl TextToDialogueClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Converts a list of text and voice ID pairs into speech (dialogue) and returns audio.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn convert(
        &self,
        request: &BodyTextToDialogueMultiVoiceV1TextToDialoguePost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/text-to-dialogue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .bool("enable_logging", request.enable_logging.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Converts a list of text and voice ID pairs into speech (dialogue) and returns an audio stream.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn stream(
        &self,
        request: &BodyTextToDialogueMultiVoiceStreamingV1TextToDialogueStreamPost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/text-to-dialogue/stream",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .bool("enable_logging", request.enable_logging.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Converts a list of text and voice ID pairs into speech (dialogue) and returns a stream of JSON blobs containing audio as a base64 encoded string and timestamps
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Complete JSON response (fetched at once, not streaming)
    pub async fn stream_with_timestamps(
        &self,
        request: &BodyTextToDialogueStreamWithTimestamps,
        options: Option<RequestOptions>,
    ) -> Result<StreamingAudioChunkWithTimestampsAndVoiceSegmentsResponseModel, ApiError> {
        self.http_client
            .execute_request::<StreamingAudioChunkWithTimestampsAndVoiceSegmentsResponseModel>(
                Method::POST,
                "v1/text-to-dialogue/stream/with-timestamps",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .bool("enable_logging", request.enable_logging.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Generate dialogue from text with precise character-level timing information for audio-text synchronization.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert_with_timestamps(
        &self,
        request: &BodyTextToDialogueFullWithTimestamps,
        options: Option<RequestOptions>,
    ) -> Result<AudioWithTimestampsAndVoiceSegmentsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/text-to-dialogue/with-timestamps",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .bool("enable_logging", request.enable_logging.clone())
                    .build(),
                options,
            )
            .await
    }
}
