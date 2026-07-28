use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod avatar;
pub use avatar::AvatarClient;
pub struct WidgetClient {
    pub http_client: HttpClient,
    pub avatar: AvatarClient,
}

impl WidgetClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            avatar: AvatarClient::new(config.clone())?,
        })
    }

    /// Retrieve the widget configuration for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `conversation_signature` - An expiring token that enables a websocket conversation to start. These can be generated for an agent using the /v1/convai/conversation/get_signed_url endpoint
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
    ///         .conversational_ai
    ///         .agents
    ///         .widget
    ///         .get(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &ConversationalAiAgentsWidgetGetQueryRequest {
    ///                 conversation_signature: Some("conversation_signature".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        agent_id: &str,
        request: &ConversationalAiAgentsWidgetGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentEmbedResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/widget", agent_id),
                None,
                QueryBuilder::new()
                    .string(
                        "conversation_signature",
                        request.conversation_signature.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }
}
