pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationHistoryEvaluationCriteriaResultCommonModel {
    #[serde(default)]
    pub criteria_id: String,
    pub result: EvaluationSuccessResult,
    #[serde(default)]
    pub rationale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_mode: Option<CriteriaScoringMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i64>,
}

impl ConversationHistoryEvaluationCriteriaResultCommonModel {
    pub fn builder() -> ConversationHistoryEvaluationCriteriaResultCommonModelBuilder {
        <ConversationHistoryEvaluationCriteriaResultCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryEvaluationCriteriaResultCommonModelBuilder {
    criteria_id: Option<String>,
    result: Option<EvaluationSuccessResult>,
    rationale: Option<String>,
    scoring_mode: Option<CriteriaScoringMode>,
    score: Option<i64>,
    max_score: Option<i64>,
}

impl ConversationHistoryEvaluationCriteriaResultCommonModelBuilder {
    pub fn criteria_id(mut self, value: impl Into<String>) -> Self {
        self.criteria_id = Some(value.into());
        self
    }

    pub fn result(mut self, value: EvaluationSuccessResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn rationale(mut self, value: impl Into<String>) -> Self {
        self.rationale = Some(value.into());
        self
    }

    pub fn scoring_mode(mut self, value: CriteriaScoringMode) -> Self {
        self.scoring_mode = Some(value);
        self
    }

    pub fn score(mut self, value: i64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn max_score(mut self, value: i64) -> Self {
        self.max_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryEvaluationCriteriaResultCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`criteria_id`](ConversationHistoryEvaluationCriteriaResultCommonModelBuilder::criteria_id)
    /// - [`result`](ConversationHistoryEvaluationCriteriaResultCommonModelBuilder::result)
    /// - [`rationale`](ConversationHistoryEvaluationCriteriaResultCommonModelBuilder::rationale)
    pub fn build(self) -> Result<ConversationHistoryEvaluationCriteriaResultCommonModel, BuildError> {
        Ok(ConversationHistoryEvaluationCriteriaResultCommonModel {
            criteria_id: self.criteria_id.ok_or_else(|| BuildError::missing_field("criteria_id"))?,
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            rationale: self.rationale.ok_or_else(|| BuildError::missing_field("rationale"))?,
            scoring_mode: self.scoring_mode,
            score: self.score,
            max_score: self.max_score,
        })
    }
}
