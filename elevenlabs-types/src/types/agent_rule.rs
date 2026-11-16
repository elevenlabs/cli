pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single behavioral rule for the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentRule {
    /// Title of the rule.
    #[serde(default)]
    pub title: StrippedStr,
    /// What the agent should do, in detail.
    #[serde(default)]
    pub description: StrippedStr,
    /// strict: the agent must always follow the rule; suggestion: a guideline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<RulePriority>,
    /// ID of the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// When the rule was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
}

impl AgentRule {
    pub fn builder() -> AgentRuleBuilder {
        <AgentRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentRuleBuilder {
    title: Option<StrippedStr>,
    description: Option<StrippedStr>,
    priority: Option<RulePriority>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
}

impl AgentRuleBuilder {
    pub fn title(mut self, value: StrippedStr) -> Self {
        self.title = Some(value);
        self
    }

    pub fn description(mut self, value: StrippedStr) -> Self {
        self.description = Some(value);
        self
    }

    pub fn priority(mut self, value: RulePriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](AgentRuleBuilder::title)
    /// - [`description`](AgentRuleBuilder::description)
    pub fn build(self) -> Result<AgentRule, BuildError> {
        Ok(AgentRule {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            priority: self.priority,
            id: self.id,
            created_at_unix: self.created_at_unix,
        })
    }
}
