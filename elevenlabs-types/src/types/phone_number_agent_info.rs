pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneNumberAgentInfo {
    /// The ID of the agent
    #[serde(default)]
    pub agent_id: String,
    /// The name of the agent
    #[serde(default)]
    pub agent_name: String,
    /// Environment to use for resolving environment variables on calls to this number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Agent branch to use for calls to this number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl PhoneNumberAgentInfo {
    pub fn builder() -> PhoneNumberAgentInfoBuilder {
        <PhoneNumberAgentInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneNumberAgentInfoBuilder {
    agent_id: Option<String>,
    agent_name: Option<String>,
    environment: Option<String>,
    branch_id: Option<String>,
}

impl PhoneNumberAgentInfoBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneNumberAgentInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](PhoneNumberAgentInfoBuilder::agent_id)
    /// - [`agent_name`](PhoneNumberAgentInfoBuilder::agent_name)
    pub fn build(self) -> Result<PhoneNumberAgentInfo, BuildError> {
        Ok(PhoneNumberAgentInfo {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_name: self.agent_name.ok_or_else(|| BuildError::missing_field("agent_name"))?,
            environment: self.environment,
            branch_id: self.branch_id,
        })
    }
}
