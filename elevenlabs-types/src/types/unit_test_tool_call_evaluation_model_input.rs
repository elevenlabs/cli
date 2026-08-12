pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UnitTestToolCallEvaluationModelInput {
    /// Parameters to evaluate for the agent's tool call. If empty, the tool call parameters are not evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<UnitTestToolCallParameter>>,
    /// The tool to evaluate a call against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tool: Option<ReferencedToolCommonModel>,
    /// Whether to verify that the tool was NOT called.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_absence: Option<bool>,
    /// Configuration for testing workflow node transitions. When set, the test will verify the agent transitions to the specified workflow node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_transition: Option<UnitTestWorkflowNodeTransitionEvaluationNodeId>,
}

impl UnitTestToolCallEvaluationModelInput {
    pub fn builder() -> UnitTestToolCallEvaluationModelInputBuilder {
        <UnitTestToolCallEvaluationModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitTestToolCallEvaluationModelInputBuilder {
    parameters: Option<Vec<UnitTestToolCallParameter>>,
    referenced_tool: Option<ReferencedToolCommonModel>,
    verify_absence: Option<bool>,
    workflow_node_transition: Option<UnitTestWorkflowNodeTransitionEvaluationNodeId>,
}

impl UnitTestToolCallEvaluationModelInputBuilder {
    pub fn parameters(mut self, value: Vec<UnitTestToolCallParameter>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn referenced_tool(mut self, value: ReferencedToolCommonModel) -> Self {
        self.referenced_tool = Some(value);
        self
    }

    pub fn verify_absence(mut self, value: bool) -> Self {
        self.verify_absence = Some(value);
        self
    }

    pub fn workflow_node_transition(mut self, value: UnitTestWorkflowNodeTransitionEvaluationNodeId) -> Self {
        self.workflow_node_transition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitTestToolCallEvaluationModelInput`].
    pub fn build(self) -> Result<UnitTestToolCallEvaluationModelInput, BuildError> {
        Ok(UnitTestToolCallEvaluationModelInput {
            parameters: self.parameters,
            referenced_tool: self.referenced_tool,
            verify_absence: self.verify_absence,
            workflow_node_transition: self.workflow_node_transition,
        })
    }
}
