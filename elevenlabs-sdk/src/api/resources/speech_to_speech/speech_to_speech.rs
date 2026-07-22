use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SpeechToSpeechClient {
    pub http_client: HttpClient,
}

impl SpeechToSpeechClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Transform audio from one voice to another. Maintain full control over emotion, timing and delivery.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    /// * `optimize_streaming_latency` - You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
    /// 0 - default mode (no latency optimizations)
    /// 1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
    /// 2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
    /// 3 - max latency optimizations
    /// 4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).
    ///
    /// Defaults to None.
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn convert(
        &self,
        voice_id: &str,
        request: &ConvertRequest2,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_multipart_stream_request(
                Method::POST,
                &format!("v1/speech-to-speech/{}", voice_id),
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .bool("enable_logging", request.enable_logging.clone())
                    .int(
                        "optimize_streaming_latency",
                        request.optimize_streaming_latency.clone(),
                    )
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Stream audio from one voice to another. Maintain full control over emotion, timing and delivery.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `enable_logging` - When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    /// * `optimize_streaming_latency` - You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
    /// 0 - default mode (no latency optimizations)
    /// 1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
    /// 2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
    /// 3 - max latency optimizations
    /// 4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).
    ///
    /// Defaults to None.
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn stream(
        &self,
        voice_id: &str,
        request: &StreamRequest2,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_multipart_stream_request(
                Method::POST,
                &format!("v1/speech-to-speech/{}/stream", voice_id),
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .bool("enable_logging", request.enable_logging.clone())
                    .int(
                        "optimize_streaming_latency",
                        request.optimize_streaming_latency.clone(),
                    )
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }
}
