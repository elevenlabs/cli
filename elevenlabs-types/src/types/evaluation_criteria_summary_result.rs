pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EvaluationCriteriaSummaryResult {
    pub result: EvaluationSuccessResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i64>,
}

impl EvaluationCriteriaSummaryResult {
    pub fn builder() -> EvaluationCriteriaSummaryResultBuilder {
        <EvaluationCriteriaSummaryResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationCriteriaSummaryResultBuilder {
    result: Option<EvaluationSuccessResult>,
    score: Option<i64>,
    max_score: Option<i64>,
}

impl EvaluationCriteriaSummaryResultBuilder {
    pub fn result(mut self, value: EvaluationSuccessResult) -> Self {
        self.result = Some(value);
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

    /// Consumes the builder and constructs a [`EvaluationCriteriaSummaryResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](EvaluationCriteriaSummaryResultBuilder::result)
    pub fn build(self) -> Result<EvaluationCriteriaSummaryResult, BuildError> {
        Ok(EvaluationCriteriaSummaryResult {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            score: self.score,
            max_score: self.max_score,
        })
    }
}
