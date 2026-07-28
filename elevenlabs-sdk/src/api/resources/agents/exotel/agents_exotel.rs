use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ExotelClient {
    pub http_client: HttpClient,
}

impl ExotelClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Handle an outbound call via Exotel Connect API
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
    ///         .agents
    ///         .exotel
    ///         .outbound_call(
    ///             &BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost {
    ///                 agent_id: "agent_id".to_string(),
    ///                 agent_phone_number_id: "agent_phone_number_id".to_string(),
    ///                 to_number: "to_number".to_string(),
    ///                 conversation_initiation_client_data: None,
    ///                 telephony_call_config: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn outbound_call(
        &self,
        request: &BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost,
        options: Option<RequestOptions>,
    ) -> Result<ExotelOutboundCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/exotel/outbound-call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
