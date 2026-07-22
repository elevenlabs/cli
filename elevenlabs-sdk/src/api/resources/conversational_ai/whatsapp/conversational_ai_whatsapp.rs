use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WhatsappClient {
    pub http_client: HttpClient,
}

impl WhatsappClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Make an outbound call via WhatsApp
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
        request: &BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost,
        options: Option<RequestOptions>,
    ) -> Result<WhatsAppOutboundCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/whatsapp/outbound-call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send an outbound message via WhatsApp
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn outbound_message(
        &self,
        request: &BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost,
        options: Option<RequestOptions>,
    ) -> Result<WhatsAppOutboundMessageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/whatsapp/outbound-message",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
