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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .pvc
    ///         .verification
    ///         .request(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &RequestRequest {
    ///                 files: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 extra_text: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
