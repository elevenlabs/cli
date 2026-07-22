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
