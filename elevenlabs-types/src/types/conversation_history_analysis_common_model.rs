pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistoryAnalysisCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria_results: Option<HashMap<String, ConversationHistoryEvaluationCriteriaResultCommonModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_results: Option<HashMap<String, DataCollectionResultCommonModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria_results_list: Option<Vec<ConversationHistoryEvaluationCriteriaResultCommonModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_results_list: Option<Vec<DataCollectionResultCommonModel>>,
    pub call_successful: EvaluationSuccessResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub call_success_score: Option<f64>,
    #[serde(default)]
    pub transcript_summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_summary_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoped: Option<Vec<ScopedAnalysisResult>>,
}

impl ConversationHistoryAnalysisCommonModel {
    pub fn builder() -> ConversationHistoryAnalysisCommonModelBuilder {
        <ConversationHistoryAnalysisCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryAnalysisCommonModelBuilder {
    evaluation_criteria_results: Option<HashMap<String, ConversationHistoryEvaluationCriteriaResultCommonModel>>,
    data_collection_results: Option<HashMap<String, DataCollectionResultCommonModel>>,
    evaluation_criteria_results_list: Option<Vec<ConversationHistoryEvaluationCriteriaResultCommonModel>>,
    data_collection_results_list: Option<Vec<DataCollectionResultCommonModel>>,
    call_successful: Option<EvaluationSuccessResult>,
    call_success_score: Option<f64>,
    transcript_summary: Option<String>,
    call_summary_title: Option<String>,
    scoped: Option<Vec<ScopedAnalysisResult>>,
}

impl ConversationHistoryAnalysisCommonModelBuilder {
    pub fn evaluation_criteria_results(mut self, value: HashMap<String, ConversationHistoryEvaluationCriteriaResultCommonModel>) -> Self {
        self.evaluation_criteria_results = Some(value);
        self
    }

    pub fn data_collection_results(mut self, value: HashMap<String, DataCollectionResultCommonModel>) -> Self {
        self.data_collection_results = Some(value);
        self
    }

    pub fn evaluation_criteria_results_list(mut self, value: Vec<ConversationHistoryEvaluationCriteriaResultCommonModel>) -> Self {
        self.evaluation_criteria_results_list = Some(value);
        self
    }

    pub fn data_collection_results_list(mut self, value: Vec<DataCollectionResultCommonModel>) -> Self {
        self.data_collection_results_list = Some(value);
        self
    }

    pub fn call_successful(mut self, value: EvaluationSuccessResult) -> Self {
        self.call_successful = Some(value);
        self
    }

    pub fn call_success_score(mut self, value: f64) -> Self {
        self.call_success_score = Some(value);
        self
    }

    pub fn transcript_summary(mut self, value: impl Into<String>) -> Self {
        self.transcript_summary = Some(value.into());
        self
    }

    pub fn call_summary_title(mut self, value: impl Into<String>) -> Self {
        self.call_summary_title = Some(value.into());
        self
    }

    pub fn scoped(mut self, value: Vec<ScopedAnalysisResult>) -> Self {
        self.scoped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryAnalysisCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_successful`](ConversationHistoryAnalysisCommonModelBuilder::call_successful)
    /// - [`transcript_summary`](ConversationHistoryAnalysisCommonModelBuilder::transcript_summary)
    pub fn build(self) -> Result<ConversationHistoryAnalysisCommonModel, BuildError> {
        Ok(ConversationHistoryAnalysisCommonModel {
            evaluation_criteria_results: self.evaluation_criteria_results,
            data_collection_results: self.data_collection_results,
            evaluation_criteria_results_list: self.evaluation_criteria_results_list,
            data_collection_results_list: self.data_collection_results_list,
            call_successful: self.call_successful.ok_or_else(|| BuildError::missing_field("call_successful"))?,
            call_success_score: self.call_success_score,
            transcript_summary: self.transcript_summary.ok_or_else(|| BuildError::missing_field("transcript_summary"))?,
            call_summary_title: self.call_summary_title,
            scoped: self.scoped,
        })
    }
}
