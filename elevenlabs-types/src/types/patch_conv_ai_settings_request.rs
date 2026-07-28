pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchConvAiSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data_webhook: Option<ConversationInitiationClientDataWebhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<ConvAiWebhooks>,
    /// Whether the workspace can use MCP servers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_mcp_servers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_retention_period_days: Option<i64>,
    /// Days to retain conversation embeddings. None means use the system default (30 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_embedding_retention_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_livekit_stack: Option<LivekitStackType>,
}

impl PatchConvAiSettingsRequest {
    pub fn builder() -> PatchConvAiSettingsRequestBuilder {
        <PatchConvAiSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchConvAiSettingsRequestBuilder {
    conversation_initiation_client_data_webhook: Option<ConversationInitiationClientDataWebhook>,
    webhooks: Option<ConvAiWebhooks>,
    can_use_mcp_servers: Option<bool>,
    rag_retention_period_days: Option<i64>,
    conversation_embedding_retention_days: Option<i64>,
    default_livekit_stack: Option<LivekitStackType>,
}

impl PatchConvAiSettingsRequestBuilder {
    pub fn conversation_initiation_client_data_webhook(mut self, value: ConversationInitiationClientDataWebhook) -> Self {
        self.conversation_initiation_client_data_webhook = Some(value);
        self
    }

    pub fn webhooks(mut self, value: ConvAiWebhooks) -> Self {
        self.webhooks = Some(value);
        self
    }

    pub fn can_use_mcp_servers(mut self, value: bool) -> Self {
        self.can_use_mcp_servers = Some(value);
        self
    }

    pub fn rag_retention_period_days(mut self, value: i64) -> Self {
        self.rag_retention_period_days = Some(value);
        self
    }

    pub fn conversation_embedding_retention_days(mut self, value: i64) -> Self {
        self.conversation_embedding_retention_days = Some(value);
        self
    }

    pub fn default_livekit_stack(mut self, value: LivekitStackType) -> Self {
        self.default_livekit_stack = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchConvAiSettingsRequest`].
    pub fn build(self) -> Result<PatchConvAiSettingsRequest, BuildError> {
        Ok(PatchConvAiSettingsRequest {
            conversation_initiation_client_data_webhook: self.conversation_initiation_client_data_webhook,
            webhooks: self.webhooks,
            can_use_mcp_servers: self.can_use_mcp_servers,
            rag_retention_period_days: self.rag_retention_period_days,
            conversation_embedding_retention_days: self.conversation_embedding_retention_days,
            default_livekit_stack: self.default_livekit_stack,
        })
    }
}

