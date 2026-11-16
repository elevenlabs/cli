pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single customer-facing receptionist agent.
/// 
/// A workspace can have multiple of these. Each entry owns its own ConvAI
/// agent id and full ``CustomerFacingConfig`` (voice, language, first message,
/// system tool toggles, agent rules, transfer rules).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionCustomerFacingAgent {
    /// ID of the ElevenLabs agent behind this receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// The receptionist's voice, language, greeting, rules and other settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ReceptionCustomerFacingConfigInput>,
    /// Unix timestamp when this receptionist was deactivated. None when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated_at_unix: Option<i64>,
    /// Why this receptionist was deactivated. None when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_reason: Option<ReceptionCustomerFacingAgentDeactivationReason>,
}

impl ReceptionCustomerFacingAgent {
    pub fn builder() -> ReceptionCustomerFacingAgentBuilder {
        <ReceptionCustomerFacingAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCustomerFacingAgentBuilder {
    agent_id: Option<String>,
    config: Option<ReceptionCustomerFacingConfigInput>,
    deactivated_at_unix: Option<i64>,
    deactivation_reason: Option<ReceptionCustomerFacingAgentDeactivationReason>,
}

impl ReceptionCustomerFacingAgentBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn config(mut self, value: ReceptionCustomerFacingConfigInput) -> Self {
        self.config = Some(value);
        self
    }

    pub fn deactivated_at_unix(mut self, value: i64) -> Self {
        self.deactivated_at_unix = Some(value);
        self
    }

    pub fn deactivation_reason(mut self, value: ReceptionCustomerFacingAgentDeactivationReason) -> Self {
        self.deactivation_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionCustomerFacingAgent`].
    pub fn build(self) -> Result<ReceptionCustomerFacingAgent, BuildError> {
        Ok(ReceptionCustomerFacingAgent {
            agent_id: self.agent_id,
            config: self.config,
            deactivated_at_unix: self.deactivated_at_unix,
            deactivation_reason: self.deactivation_reason,
        })
    }
}
