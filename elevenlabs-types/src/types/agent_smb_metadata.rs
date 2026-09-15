pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Identifies an agent as part of the SMB product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSmbMetadata {
    /// Distinguishes between the customer-facing voice agent and the internal assistant agent.
    pub agent_type: SmbAgentType,
    /// True for pre-signup onboarding agents that have not yet been transferred to a user workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ephemeral: Option<bool>,
    /// Hash of the SMB data+code inputs this agent was last regenerated against; the conversation-start drift gate. None (unstamped) regenerates once on the next conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_fingerprint: Option<String>,
}

impl AgentSmbMetadata {
    pub fn builder() -> AgentSmbMetadataBuilder {
        <AgentSmbMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSmbMetadataBuilder {
    agent_type: Option<SmbAgentType>,
    is_ephemeral: Option<bool>,
    source_fingerprint: Option<String>,
}

impl AgentSmbMetadataBuilder {
    pub fn agent_type(mut self, value: SmbAgentType) -> Self {
        self.agent_type = Some(value);
        self
    }

    pub fn is_ephemeral(mut self, value: bool) -> Self {
        self.is_ephemeral = Some(value);
        self
    }

    pub fn source_fingerprint(mut self, value: impl Into<String>) -> Self {
        self.source_fingerprint = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSmbMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_type`](AgentSmbMetadataBuilder::agent_type)
    pub fn build(self) -> Result<AgentSmbMetadata, BuildError> {
        Ok(AgentSmbMetadata {
            agent_type: self.agent_type.ok_or_else(|| BuildError::missing_field("agent_type"))?,
            is_ephemeral: self.is_ephemeral,
            source_fingerprint: self.source_fingerprint,
        })
    }
}
