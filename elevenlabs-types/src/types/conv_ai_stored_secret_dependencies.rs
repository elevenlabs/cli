pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConvAiStoredSecretDependencies {
    #[serde(default)]
    pub tools: Vec<ConvAiStoredSecretDependenciesToolsItem>,
    /// Whether there are more tool dependents beyond the returned preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_has_more: Option<bool>,
    #[serde(default)]
    pub agents: Vec<ConvAiStoredSecretDependenciesAgentsItem>,
    /// Whether there are more agent dependents beyond the returned preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<DependentPhoneNumberIdentifier>>,
    /// Whether there are more phone number dependents beyond the returned preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers_has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_servers: Option<Vec<ConvAiStoredSecretDependenciesMcpServersItem>>,
    #[serde(default)]
    pub others: Vec<SecretDependencyType>,
}

impl ConvAiStoredSecretDependencies {
    pub fn builder() -> ConvAiStoredSecretDependenciesBuilder {
        <ConvAiStoredSecretDependenciesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiStoredSecretDependenciesBuilder {
    tools: Option<Vec<ConvAiStoredSecretDependenciesToolsItem>>,
    tools_has_more: Option<bool>,
    agents: Option<Vec<ConvAiStoredSecretDependenciesAgentsItem>>,
    agents_has_more: Option<bool>,
    phone_numbers: Option<Vec<DependentPhoneNumberIdentifier>>,
    phone_numbers_has_more: Option<bool>,
    mcp_servers: Option<Vec<ConvAiStoredSecretDependenciesMcpServersItem>>,
    others: Option<Vec<SecretDependencyType>>,
}

impl ConvAiStoredSecretDependenciesBuilder {
    pub fn tools(mut self, value: Vec<ConvAiStoredSecretDependenciesToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tools_has_more(mut self, value: bool) -> Self {
        self.tools_has_more = Some(value);
        self
    }

    pub fn agents(mut self, value: Vec<ConvAiStoredSecretDependenciesAgentsItem>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn agents_has_more(mut self, value: bool) -> Self {
        self.agents_has_more = Some(value);
        self
    }

    pub fn phone_numbers(mut self, value: Vec<DependentPhoneNumberIdentifier>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    pub fn phone_numbers_has_more(mut self, value: bool) -> Self {
        self.phone_numbers_has_more = Some(value);
        self
    }

    pub fn mcp_servers(mut self, value: Vec<ConvAiStoredSecretDependenciesMcpServersItem>) -> Self {
        self.mcp_servers = Some(value);
        self
    }

    pub fn others(mut self, value: Vec<SecretDependencyType>) -> Self {
        self.others = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConvAiStoredSecretDependencies`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tools`](ConvAiStoredSecretDependenciesBuilder::tools)
    /// - [`agents`](ConvAiStoredSecretDependenciesBuilder::agents)
    /// - [`others`](ConvAiStoredSecretDependenciesBuilder::others)
    pub fn build(self) -> Result<ConvAiStoredSecretDependencies, BuildError> {
        Ok(ConvAiStoredSecretDependencies {
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
            tools_has_more: self.tools_has_more,
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            agents_has_more: self.agents_has_more,
            phone_numbers: self.phone_numbers,
            phone_numbers_has_more: self.phone_numbers_has_more,
            mcp_servers: self.mcp_servers,
            others: self.others.ok_or_else(|| BuildError::missing_field("others"))?,
        })
    }
}
