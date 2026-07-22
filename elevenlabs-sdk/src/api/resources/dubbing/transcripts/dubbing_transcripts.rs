use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TranscriptsClient {
    pub http_client: HttpClient,
}

impl TranscriptsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Fetch the transcript for one of the languages in a dub.
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
    pub async fn get(
        &self,
        dubbing_id: &str,
        language_code: &str,
        format_type: &TranscriptsGetRequestFormatType,
        options: Option<RequestOptions>,
    ) -> Result<DubbingTranscriptsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/dubbing/{}/transcripts/{}/format/{}",
                    dubbing_id, language_code, format_type
                ),
                None,
                None,
                options,
            )
            .await
    }
}
