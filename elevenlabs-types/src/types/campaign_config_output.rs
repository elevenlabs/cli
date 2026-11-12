pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignConfigOutput {
    /// The policy campaigns take unless they set their own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<PolicyConfigOutput>,
    /// Overrides of `policy`, keyed by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_policies: Option<HashMap<String, AgentPolicyOverrideOutput>>,
    /// Overrides for WhatsApp voice campaigns, layered between the workspace policy and an agent's overrides. Sections left out follow WhatsApp voice's built-in defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_voice_policy: Option<AgentPolicyOverrideOutput>,
}

impl CampaignConfigOutput {
    pub fn builder() -> CampaignConfigOutputBuilder {
        <CampaignConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignConfigOutputBuilder {
    policy: Option<PolicyConfigOutput>,
    agent_policies: Option<HashMap<String, AgentPolicyOverrideOutput>>,
    whatsapp_voice_policy: Option<AgentPolicyOverrideOutput>,
}

impl CampaignConfigOutputBuilder {
    pub fn policy(mut self, value: PolicyConfigOutput) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn agent_policies(mut self, value: HashMap<String, AgentPolicyOverrideOutput>) -> Self {
        self.agent_policies = Some(value);
        self
    }

    pub fn whatsapp_voice_policy(mut self, value: AgentPolicyOverrideOutput) -> Self {
        self.whatsapp_voice_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignConfigOutput`].
    pub fn build(self) -> Result<CampaignConfigOutput, BuildError> {
        Ok(CampaignConfigOutput {
            policy: self.policy,
            agent_policies: self.agent_policies,
            whatsapp_voice_policy: self.whatsapp_voice_policy,
        })
    }
}
