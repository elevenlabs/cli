pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignConfigInput {
    /// The policy campaigns take unless they set their own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<PolicyConfigInput>,
    /// Overrides of `policy`, keyed by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_policies: Option<HashMap<String, AgentPolicyOverrideInput>>,
    /// Overrides for WhatsApp voice campaigns, layered between the workspace policy and an agent's overrides. Sections left out follow WhatsApp voice's built-in defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_voice_policy: Option<AgentPolicyOverrideInput>,
}

impl CampaignConfigInput {
    pub fn builder() -> CampaignConfigInputBuilder {
        <CampaignConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignConfigInputBuilder {
    policy: Option<PolicyConfigInput>,
    agent_policies: Option<HashMap<String, AgentPolicyOverrideInput>>,
    whatsapp_voice_policy: Option<AgentPolicyOverrideInput>,
}

impl CampaignConfigInputBuilder {
    pub fn policy(mut self, value: PolicyConfigInput) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn agent_policies(mut self, value: HashMap<String, AgentPolicyOverrideInput>) -> Self {
        self.agent_policies = Some(value);
        self
    }

    pub fn whatsapp_voice_policy(mut self, value: AgentPolicyOverrideInput) -> Self {
        self.whatsapp_voice_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignConfigInput`].
    pub fn build(self) -> Result<CampaignConfigInput, BuildError> {
        Ok(CampaignConfigInput {
            policy: self.policy,
            agent_policies: self.agent_policies,
            whatsapp_voice_policy: self.whatsapp_voice_policy,
        })
    }
}
