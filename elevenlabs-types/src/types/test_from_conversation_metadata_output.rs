pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TestFromConversationMetadataOutput {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_agent_reply: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
}

impl TestFromConversationMetadataOutput {
    pub fn builder() -> TestFromConversationMetadataOutputBuilder {
        <TestFromConversationMetadataOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestFromConversationMetadataOutputBuilder {
    conversation_id: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    workflow_node_id: Option<String>,
    original_agent_reply: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
}

impl TestFromConversationMetadataOutputBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_id = Some(value.into());
        self
    }

    pub fn original_agent_reply(mut self, value: Vec<ConversationHistoryTranscriptCommonModelOutput>) -> Self {
        self.original_agent_reply = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestFromConversationMetadataOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](TestFromConversationMetadataOutputBuilder::conversation_id)
    /// - [`agent_id`](TestFromConversationMetadataOutputBuilder::agent_id)
    pub fn build(self) -> Result<TestFromConversationMetadataOutput, BuildError> {
        Ok(TestFromConversationMetadataOutput {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            workflow_node_id: self.workflow_node_id,
            original_agent_reply: self.original_agent_reply,
        })
    }
}
