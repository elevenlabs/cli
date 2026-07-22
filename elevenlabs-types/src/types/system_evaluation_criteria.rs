pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemEvaluationCriteria {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub user_sentiment_score: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub user_frustration_score: f64,
}

impl SystemEvaluationCriteria {
    pub fn builder() -> SystemEvaluationCriteriaBuilder {
        <SystemEvaluationCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemEvaluationCriteriaBuilder {
    user_sentiment_score: Option<f64>,
    user_frustration_score: Option<f64>,
}

impl SystemEvaluationCriteriaBuilder {
    pub fn user_sentiment_score(mut self, value: f64) -> Self {
        self.user_sentiment_score = Some(value);
        self
    }

    pub fn user_frustration_score(mut self, value: f64) -> Self {
        self.user_frustration_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemEvaluationCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_sentiment_score`](SystemEvaluationCriteriaBuilder::user_sentiment_score)
    /// - [`user_frustration_score`](SystemEvaluationCriteriaBuilder::user_frustration_score)
    pub fn build(self) -> Result<SystemEvaluationCriteria, BuildError> {
        Ok(SystemEvaluationCriteria {
            user_sentiment_score: self.user_sentiment_score.ok_or_else(|| BuildError::missing_field("user_sentiment_score"))?,
            user_frustration_score: self.user_frustration_score.ok_or_else(|| BuildError::missing_field("user_frustration_score"))?,
        })
    }
}
