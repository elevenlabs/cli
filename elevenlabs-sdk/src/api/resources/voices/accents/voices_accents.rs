use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AccentsClient {
    pub http_client: HttpClient,
}

impl AccentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets the list of available accents in the shared voice library.
    ///
    /// # Arguments
    ///
    /// * `language` - If provided, only accents for this language code are returned.
    /// * `model_id` - If provided, returns the accents available for this model. Defaults to the most complete accent list when omitted.
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
    ///         .accents
    ///         .get(
    ///             &VoicesAccentsGetQueryRequest {
    ///                 language: Some("language".to_string()),
    ///                 model_id: Some("model_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &VoicesAccentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetVoiceAccentsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/voices/accents",
                None,
                QueryBuilder::new()
                    .string("language", request.language.clone())
                    .string("model_id", request.model_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
