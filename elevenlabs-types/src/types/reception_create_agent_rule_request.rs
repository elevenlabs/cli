pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionCreateAgentRuleRequest {
    /// Title of the rule.
    #[serde(default)]
    pub title: StrippedStr,
    /// What the agent should do, in detail.
    #[serde(default)]
    pub description: StrippedStr,
    /// strict: the agent must always follow the rule; suggestion: a guideline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<RulePriority>,
}

impl ReceptionCreateAgentRuleRequest {
    pub fn builder() -> ReceptionCreateAgentRuleRequestBuilder {
        <ReceptionCreateAgentRuleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCreateAgentRuleRequestBuilder {
    title: Option<StrippedStr>,
    description: Option<StrippedStr>,
    priority: Option<RulePriority>,
}

impl ReceptionCreateAgentRuleRequestBuilder {
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

    /// Consumes the builder and constructs a [`ReceptionCreateAgentRuleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](ReceptionCreateAgentRuleRequestBuilder::title)
    /// - [`description`](ReceptionCreateAgentRuleRequestBuilder::description)
    pub fn build(self) -> Result<ReceptionCreateAgentRuleRequest, BuildError> {
        Ok(ReceptionCreateAgentRuleRequest {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            priority: self.priority,
        })
    }
}

