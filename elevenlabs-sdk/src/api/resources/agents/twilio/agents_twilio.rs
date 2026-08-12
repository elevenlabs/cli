use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TwilioClient {
    pub http_client: HttpClient,
}

impl TwilioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Handle an outbound call via Twilio
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
    ///         .twilio
    ///         .outbound_call(
    ///             &BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost {
    ///                 agent_id: "agent_id".to_string(),
    ///                 agent_phone_number_id: "agent_phone_number_id".to_string(),
    ///                 to_number: "to_number".to_string(),
    ///                 conversation_initiation_client_data: None,
    ///                 call_recording_enabled: None,
    ///                 telephony_call_config: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn outbound_call(
        &self,
        request: &BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost,
        options: Option<RequestOptions>,
    ) -> Result<TwilioOutboundCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/twilio/outbound-call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Register a Twilio call and return TwiML to connect the call
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
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
    ///         .twilio
    ///         .register_call(
    ///             &BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost {
    ///                 agent_id: "agent_id".to_string(),
    ///                 from_number: "from_number".to_string(),
    ///                 to_number: "to_number".to_string(),
    ///                 direction: None,
    ///                 conversation_initiation_client_data: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn register_call(
        &self,
        request: &BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/twilio/register-call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
