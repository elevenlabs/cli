pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentAnalysisItemsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria: Option<Vec<AgentAnalysisItemsInputEvaluationCriteriaItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<Vec<AgentAnalysisItemsInputDataCollectionItem>>,
}

impl AgentAnalysisItemsInput {
    pub fn builder() -> AgentAnalysisItemsInputBuilder {
        <AgentAnalysisItemsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentAnalysisItemsInputBuilder {
    evaluation_criteria: Option<Vec<AgentAnalysisItemsInputEvaluationCriteriaItem>>,
    data_collection: Option<Vec<AgentAnalysisItemsInputDataCollectionItem>>,
}

impl AgentAnalysisItemsInputBuilder {
    pub fn evaluation_criteria(mut self, value: Vec<AgentAnalysisItemsInputEvaluationCriteriaItem>) -> Self {
        self.evaluation_criteria = Some(value);
        self
    }

    pub fn data_collection(mut self, value: Vec<AgentAnalysisItemsInputDataCollectionItem>) -> Self {
        self.data_collection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentAnalysisItemsInput`].
    pub fn build(self) -> Result<AgentAnalysisItemsInput, BuildError> {
        Ok(AgentAnalysisItemsInput {
            evaluation_criteria: self.evaluation_criteria,
            data_collection: self.data_collection,
        })
    }
}
