pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An agent (and optional branch) that participated in the call, in first-seen transcript order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VisitedAgentRef {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl VisitedAgentRef {
    pub fn builder() -> VisitedAgentRefBuilder {
        <VisitedAgentRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VisitedAgentRefBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
}

impl VisitedAgentRefBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VisitedAgentRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](VisitedAgentRefBuilder::agent_id)
    pub fn build(self) -> Result<VisitedAgentRef, BuildError> {
        Ok(VisitedAgentRef {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
        })
    }
}
