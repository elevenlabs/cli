use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct IvcClient {
    pub http_client: HttpClient,
}

impl IvcClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a voice clone and add it to your Voices
    ///
    /// # Arguments
    ///
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
    ///         .ivc
    ///         .create(
    ///             &CreateRequest {
    ///                 files: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 name: "name".to_string(),
    ///                 remove_background_noise: None,
    ///                 description: None,
    ///                 labels: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceIvcResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/voices/add",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
