use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod composition_plan;
pub use composition_plan::CompositionPlanClient;
pub mod finetunes;
pub use finetunes::FinetunesClient;
pub struct MusicClient {
    pub http_client: HttpClient,
    pub composition_plan: CompositionPlanClient,
    pub finetunes: FinetunesClient,
}

impl MusicClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            composition_plan: CompositionPlanClient::new(config.clone())?,
            finetunes: FinetunesClient::new(config.clone())?,
        })
    }

    /// Generate background music from one or more video files. Videos are combined in order. Optional description and style tags influence the generated music.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn video_to_music(
        &self,
        request: &VideoToMusicRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_multipart_stream_request(
                Method::POST,
                "v1/music/video-to-music",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Compose a song from a prompt or a composition plan.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn compose(
        &self,
        request: &BodyComposeMusicV1MusicPost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/music",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Compose a song from a prompt or a composition plan.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn compose_detailed(
        &self,
        request: &BodyComposeMusicWithADetailedResponseV1MusicDetailedPost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/music/detailed",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Stream a song and its detailed metadata using Server-Sent Events (SSE).
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Complete JSON response (fetched at once, not streaming)
    pub async fn compose_detailed_stream(
        &self,
        request: &BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPost,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request::<String>(
                Method::POST,
                "v1/music/detailed/stream",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Stream a composed song from a prompt or a composition plan.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn stream(
        &self,
        request: &BodyStreamComposedMusicV1MusicStreamPost,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "v1/music/stream",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upload a music file to be later used for inpainting. Price for uploading is the same as the one for song generation. All uploaded content gets inspected for copyright infringement. If copyrighted content is detected, half of the request cost is still charged.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upload(
        &self,
        request: &UploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<MusicUploadResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/music/upload",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Separate an audio file into individual stems. This endpoint might have high latency, depending on the length of the audio file.
    ///
    /// # Arguments
    ///
    /// * `output_format` - Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn separate_stems(
        &self,
        request: &SeparateStemsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_multipart_stream_request(
                Method::POST,
                "v1/music/stem-separation",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("output_format", request.output_format.clone())
                    .build(),
                options,
            )
            .await
    }
}
