pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentAnalysisItemsOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria: Option<Vec<AgentAnalysisItemsOutputEvaluationCriteriaItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<Vec<AgentAnalysisItemsOutputDataCollectionItem>>,
}

impl AgentAnalysisItemsOutput {
    pub fn builder() -> AgentAnalysisItemsOutputBuilder {
        <AgentAnalysisItemsOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentAnalysisItemsOutputBuilder {
    evaluation_criteria: Option<Vec<AgentAnalysisItemsOutputEvaluationCriteriaItem>>,
    data_collection: Option<Vec<AgentAnalysisItemsOutputDataCollectionItem>>,
}

impl AgentAnalysisItemsOutputBuilder {
    pub fn evaluation_criteria(mut self, value: Vec<AgentAnalysisItemsOutputEvaluationCriteriaItem>) -> Self {
        self.evaluation_criteria = Some(value);
        self
    }

    pub fn data_collection(mut self, value: Vec<AgentAnalysisItemsOutputDataCollectionItem>) -> Self {
        self.data_collection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentAnalysisItemsOutput`].
    pub fn build(self) -> Result<AgentAnalysisItemsOutput, BuildError> {
        Ok(AgentAnalysisItemsOutput {
            evaluation_criteria: self.evaluation_criteria,
            data_collection: self.data_collection,
        })
    }
}
