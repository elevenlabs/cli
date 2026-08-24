pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallMcpDetails {
    #[serde(default)]
    pub mcp_server_id: String,
    #[serde(default)]
    pub mcp_server_name: String,
    #[serde(default)]
    pub integration_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, String>>,
    #[serde(default)]
    pub approval_policy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_approval: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_tool_description: Option<String>,
}

impl ConversationHistoryTranscriptToolCallMcpDetails {
    pub fn builder() -> ConversationHistoryTranscriptToolCallMcpDetailsBuilder {
        <ConversationHistoryTranscriptToolCallMcpDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallMcpDetailsBuilder {
    mcp_server_id: Option<String>,
    mcp_server_name: Option<String>,
    integration_type: Option<String>,
    parameters: Option<HashMap<String, String>>,
    approval_policy: Option<String>,
    requires_approval: Option<bool>,
    mcp_tool_name: Option<String>,
    mcp_tool_description: Option<String>,
}

impl ConversationHistoryTranscriptToolCallMcpDetailsBuilder {
    pub fn mcp_server_id(mut self, value: impl Into<String>) -> Self {
        self.mcp_server_id = Some(value.into());
        self
    }

    pub fn mcp_server_name(mut self, value: impl Into<String>) -> Self {
        self.mcp_server_name = Some(value.into());
        self
    }

    pub fn integration_type(mut self, value: impl Into<String>) -> Self {
        self.integration_type = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: HashMap<String, String>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn approval_policy(mut self, value: impl Into<String>) -> Self {
        self.approval_policy = Some(value.into());
        self
    }

    pub fn requires_approval(mut self, value: bool) -> Self {
        self.requires_approval = Some(value);
        self
    }

    pub fn mcp_tool_name(mut self, value: impl Into<String>) -> Self {
        self.mcp_tool_name = Some(value.into());
        self
    }

    pub fn mcp_tool_description(mut self, value: impl Into<String>) -> Self {
        self.mcp_tool_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallMcpDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mcp_server_id`](ConversationHistoryTranscriptToolCallMcpDetailsBuilder::mcp_server_id)
    /// - [`mcp_server_name`](ConversationHistoryTranscriptToolCallMcpDetailsBuilder::mcp_server_name)
    /// - [`integration_type`](ConversationHistoryTranscriptToolCallMcpDetailsBuilder::integration_type)
    /// - [`approval_policy`](ConversationHistoryTranscriptToolCallMcpDetailsBuilder::approval_policy)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallMcpDetails, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallMcpDetails {
            mcp_server_id: self.mcp_server_id.ok_or_else(|| BuildError::missing_field("mcp_server_id"))?,
            mcp_server_name: self.mcp_server_name.ok_or_else(|| BuildError::missing_field("mcp_server_name"))?,
            integration_type: self.integration_type.ok_or_else(|| BuildError::missing_field("integration_type"))?,
            parameters: self.parameters,
            approval_policy: self.approval_policy.ok_or_else(|| BuildError::missing_field("approval_policy"))?,
            requires_approval: self.requires_approval,
            mcp_tool_name: self.mcp_tool_name,
            mcp_tool_description: self.mcp_tool_description,
        })
    }
}
