pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPost {
    /// A specification detailing how the conversation should be simulated
    #[serde(default)]
    pub simulation_specification: ConversationSimulationSpecification,
    /// A list of evaluation criteria to test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_evaluation_criteria: Option<Vec<PromptEvaluationCriteria>>,
    /// Maximum number of new turns to generate in the conversation simulation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_turns_limit: Option<i64>,
}

impl BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPost {
    pub fn builder() -> BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPostBuilder {
        <BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPostBuilder {
    simulation_specification: Option<ConversationSimulationSpecification>,
    extra_evaluation_criteria: Option<Vec<PromptEvaluationCriteria>>,
    new_turns_limit: Option<i64>,
}

impl BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPostBuilder {
    pub fn simulation_specification(mut self, value: ConversationSimulationSpecification) -> Self {
        self.simulation_specification = Some(value);
        self
    }

    pub fn extra_evaluation_criteria(mut self, value: Vec<PromptEvaluationCriteria>) -> Self {
        self.extra_evaluation_criteria = Some(value);
        self
    }

    pub fn new_turns_limit(mut self, value: i64) -> Self {
        self.new_turns_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`simulation_specification`](BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPostBuilder::simulation_specification)
    pub fn build(self) -> Result<BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPost, BuildError> {
        Ok(BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPost {
            simulation_specification: self.simulation_specification.ok_or_else(|| BuildError::missing_field("simulation_specification"))?,
            extra_evaluation_criteria: self.extra_evaluation_criteria,
            new_turns_limit: self.new_turns_limit,
        })
    }
}

