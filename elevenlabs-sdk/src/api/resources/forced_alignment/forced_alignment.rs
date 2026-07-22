use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ForcedAlignmentClient {
    pub http_client: HttpClient,
}

impl ForcedAlignmentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Force align an audio file to text. Use this endpoint to get the timing information for each character and word in an audio file based on a provided text transcript.
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
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<ForcedAlignmentResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/forced-alignment",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
