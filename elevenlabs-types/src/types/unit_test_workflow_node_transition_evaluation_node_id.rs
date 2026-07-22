pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitTestWorkflowNodeTransitionEvaluationNodeId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The ID of the agent whose workflow contains the target node.
    #[serde(default)]
    pub agent_id: String,
    /// The ID of the workflow node that the agent should transition to.
    #[serde(default)]
    pub target_node_id: String,
}

impl UnitTestWorkflowNodeTransitionEvaluationNodeId {
    pub fn builder() -> UnitTestWorkflowNodeTransitionEvaluationNodeIdBuilder {
        <UnitTestWorkflowNodeTransitionEvaluationNodeIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitTestWorkflowNodeTransitionEvaluationNodeIdBuilder {
    r#type: Option<String>,
    agent_id: Option<String>,
    target_node_id: Option<String>,
}

impl UnitTestWorkflowNodeTransitionEvaluationNodeIdBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn target_node_id(mut self, value: impl Into<String>) -> Self {
        self.target_node_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitTestWorkflowNodeTransitionEvaluationNodeId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](UnitTestWorkflowNodeTransitionEvaluationNodeIdBuilder::agent_id)
    /// - [`target_node_id`](UnitTestWorkflowNodeTransitionEvaluationNodeIdBuilder::target_node_id)
    pub fn build(self) -> Result<UnitTestWorkflowNodeTransitionEvaluationNodeId, BuildError> {
        Ok(UnitTestWorkflowNodeTransitionEvaluationNodeId {
            r#type: self.r#type,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            target_node_id: self.target_node_id.ok_or_else(|| BuildError::missing_field("target_node_id"))?,
        })
    }
}
