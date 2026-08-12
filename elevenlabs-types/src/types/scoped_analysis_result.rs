pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopedAnalysisResult {
    /// The scope of the analysis. 'conversation' uses the full transcript; 'agent' uses only the portion where the defining agent was active.
    pub scope: AnalysisScope,
    #[serde(default)]
    pub source_agent_id: String,
    /// Branch of the agent for this scoped block; disambiguates repeated agent_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria_results: Option<HashMap<String, ConversationHistoryEvaluationCriteriaResultCommonModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_results: Option<HashMap<String, DataCollectionResultCommonModel>>,
    pub successful: EvaluationSuccessResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub success_score: Option<f64>,
}

impl ScopedAnalysisResult {
    pub fn builder() -> ScopedAnalysisResultBuilder {
        <ScopedAnalysisResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScopedAnalysisResultBuilder {
    scope: Option<AnalysisScope>,
    source_agent_id: Option<String>,
    source_branch_id: Option<String>,
    evaluation_criteria_results: Option<HashMap<String, ConversationHistoryEvaluationCriteriaResultCommonModel>>,
    data_collection_results: Option<HashMap<String, DataCollectionResultCommonModel>>,
    successful: Option<EvaluationSuccessResult>,
    success_score: Option<f64>,
}

impl ScopedAnalysisResultBuilder {
    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn source_agent_id(mut self, value: impl Into<String>) -> Self {
        self.source_agent_id = Some(value.into());
        self
    }

    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    pub fn evaluation_criteria_results(mut self, value: HashMap<String, ConversationHistoryEvaluationCriteriaResultCommonModel>) -> Self {
        self.evaluation_criteria_results = Some(value);
        self
    }

    pub fn data_collection_results(mut self, value: HashMap<String, DataCollectionResultCommonModel>) -> Self {
        self.data_collection_results = Some(value);
        self
    }

    pub fn successful(mut self, value: EvaluationSuccessResult) -> Self {
        self.successful = Some(value);
        self
    }

    pub fn success_score(mut self, value: f64) -> Self {
        self.success_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScopedAnalysisResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scope`](ScopedAnalysisResultBuilder::scope)
    /// - [`source_agent_id`](ScopedAnalysisResultBuilder::source_agent_id)
    /// - [`successful`](ScopedAnalysisResultBuilder::successful)
    pub fn build(self) -> Result<ScopedAnalysisResult, BuildError> {
        Ok(ScopedAnalysisResult {
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            source_agent_id: self.source_agent_id.ok_or_else(|| BuildError::missing_field("source_agent_id"))?,
            source_branch_id: self.source_branch_id,
            evaluation_criteria_results: self.evaluation_criteria_results,
            data_collection_results: self.data_collection_results,
            successful: self.successful.ok_or_else(|| BuildError::missing_field("successful"))?,
            success_score: self.success_score,
        })
    }
}
