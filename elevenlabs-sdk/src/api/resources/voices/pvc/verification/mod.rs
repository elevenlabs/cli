use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod captcha;
pub use captcha::CaptchaClient;
pub struct VerificationClient {
    pub http_client: HttpClient,
    pub captcha: CaptchaClient,
}

impl VerificationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            captcha: CaptchaClient::new(config.clone())?,
        })
    }

    /// Request manual verification for a PVC voice.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn request(
        &self,
        voice_id: &str,
        request: &RequestRequest,
        options: Option<RequestOptions>,
    ) -> Result<RequestPvcManualVerificationResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/voices/pvc/{}/verification", voice_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
