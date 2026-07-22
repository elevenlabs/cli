use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SamplesClient {
    pub http_client: HttpClient,
}

impl SamplesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Removes a sample by its ID.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `sample_id` - ID of the sample to be used. You can use the [Get voices](/docs/api-reference/voices/get) endpoint list all the available samples for a voice.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        voice_id: &str,
        sample_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteSampleResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/voices/{}/samples/{}", voice_id, sample_id),
                None,
                None,
                options,
            )
            .await
    }
}
