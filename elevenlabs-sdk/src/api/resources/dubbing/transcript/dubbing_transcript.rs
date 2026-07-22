use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TranscriptClient {
    pub http_client: HttpClient,
}

impl TranscriptClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns transcript for the dub as an SRT or WEBVTT file.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `language_code` - ISO-693 language code to retrieve the transcript for. Use 'source' to fetch the transcript of the original media.
    /// * `format_type` - Format to return transcript in. For subtitles use either 'srt' or 'webvtt', and for a full transcript use 'json'. The 'json' format is not yet supported for Dubbing Studio.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_transcript_for_dub(
        &self,
        dubbing_id: &str,
        language_code: &str,
        request: &GetTranscriptForDubQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TranscriptGetTranscriptForDubResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/{}/transcript/{}", dubbing_id, language_code),
                None,
                QueryBuilder::new()
                    .serialize("format_type", request.format_type.clone())
                    .build(),
                options,
            )
            .await
    }
}
