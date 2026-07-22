pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentWorkflowResponseModel {
    #[serde(default)]
    pub edges: HashMap<String, WorkflowEdgeModelOutput>,
    #[serde(default)]
    pub nodes: HashMap<String, AgentWorkflowResponseModelNodesValue>,
    /// Whether to prevent loops in the workflow execution.
    #[serde(default)]
    pub prevent_subagent_loops: bool,
}

impl AgentWorkflowResponseModel {
    pub fn builder() -> AgentWorkflowResponseModelBuilder {
        <AgentWorkflowResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentWorkflowResponseModelBuilder {
    edges: Option<HashMap<String, WorkflowEdgeModelOutput>>,
    nodes: Option<HashMap<String, AgentWorkflowResponseModelNodesValue>>,
    prevent_subagent_loops: Option<bool>,
}

impl AgentWorkflowResponseModelBuilder {
    pub fn edges(mut self, value: HashMap<String, WorkflowEdgeModelOutput>) -> Self {
        self.edges = Some(value);
        self
    }

    pub fn nodes(mut self, value: HashMap<String, AgentWorkflowResponseModelNodesValue>) -> Self {
        self.nodes = Some(value);
        self
    }

    pub fn prevent_subagent_loops(mut self, value: bool) -> Self {
        self.prevent_subagent_loops = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentWorkflowResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`edges`](AgentWorkflowResponseModelBuilder::edges)
    /// - [`nodes`](AgentWorkflowResponseModelBuilder::nodes)
    /// - [`prevent_subagent_loops`](AgentWorkflowResponseModelBuilder::prevent_subagent_loops)
    pub fn build(self) -> Result<AgentWorkflowResponseModel, BuildError> {
        Ok(AgentWorkflowResponseModel {
            edges: self.edges.ok_or_else(|| BuildError::missing_field("edges"))?,
            nodes: self.nodes.ok_or_else(|| BuildError::missing_field("nodes"))?,
            prevent_subagent_loops: self.prevent_subagent_loops.ok_or_else(|| BuildError::missing_field("prevent_subagent_loops"))?,
        })
    }
}
