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
    ///         .whatsapp
    ///         .outbound_call(
    ///             &BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost {
    ///                 whatsapp_phone_number_id: "whatsapp_phone_number_id".to_string(),
    ///                 whatsapp_user_id: "whatsapp_user_id".to_string(),
    ///                 whatsapp_call_permission_request_template_name:
    ///                     "whatsapp_call_permission_request_template_name".to_string(),
    ///                 whatsapp_call_permission_request_template_language_code:
    ///                     "whatsapp_call_permission_request_template_language_code".to_string(),
    ///                 agent_id: "agent_id".to_string(),
    ///                 conversation_initiation_client_data: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///     client.agents.whatsapp.outbound_message(&BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost {
    ///         whatsapp_phone_number_id: "whatsapp_phone_number_id".to_string(),
    ///         whatsapp_user_id: "whatsapp_user_id".to_string(),
    ///         template_name: "template_name".to_string(),
    ///         template_language_code: "template_language_code".to_string(),
    ///         template_params: vec![BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostTemplateParamsItem::Body {
    ///             data: WhatsAppTemplateBodyComponentParams {
    ///                 parameters: vec![WhatsAppTemplateTextParam {
    ///                     text: "text".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             }
    ///         }],
    ///         agent_id: "agent_id".to_string(),
    ///         conversation_initiation_client_data: None
    ///     }, None).await;
    /// }
    /// ```
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
