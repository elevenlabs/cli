pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewResponseModel {
    pub review_status: ReviewResponseModelReviewStatus,
    #[serde(default)]
    pub reviewed_at_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reasons: Option<Vec<ReviewResponseModelRejectReasonsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores_breakdown: Option<HashMap<String, Option<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

impl ReviewResponseModel {
    pub fn builder() -> ReviewResponseModelBuilder {
        <ReviewResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewResponseModelBuilder {
    review_status: Option<ReviewResponseModelReviewStatus>,
    reviewed_at_unix: Option<i64>,
    reviewed_by: Option<String>,
    reject_reasons: Option<Vec<ReviewResponseModelRejectReasonsItem>>,
    scores_breakdown: Option<HashMap<String, Option<i64>>>,
    rejected_details: Option<String>,
    explanation: Option<String>,
}

impl ReviewResponseModelBuilder {
    pub fn review_status(mut self, value: ReviewResponseModelReviewStatus) -> Self {
        self.review_status = Some(value);
        self
    }

    pub fn reviewed_at_unix(mut self, value: i64) -> Self {
        self.reviewed_at_unix = Some(value);
        self
    }

    pub fn reviewed_by(mut self, value: impl Into<String>) -> Self {
        self.reviewed_by = Some(value.into());
        self
    }

    pub fn reject_reasons(mut self, value: Vec<ReviewResponseModelRejectReasonsItem>) -> Self {
        self.reject_reasons = Some(value);
        self
    }

    pub fn scores_breakdown(mut self, value: HashMap<String, Option<i64>>) -> Self {
        self.scores_breakdown = Some(value);
        self
    }

    pub fn rejected_details(mut self, value: impl Into<String>) -> Self {
        self.rejected_details = Some(value.into());
        self
    }

    pub fn explanation(mut self, value: impl Into<String>) -> Self {
        self.explanation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReviewResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`review_status`](ReviewResponseModelBuilder::review_status)
    /// - [`reviewed_at_unix`](ReviewResponseModelBuilder::reviewed_at_unix)
    pub fn build(self) -> Result<ReviewResponseModel, BuildError> {
        Ok(ReviewResponseModel {
            review_status: self.review_status.ok_or_else(|| BuildError::missing_field("review_status"))?,
            reviewed_at_unix: self.reviewed_at_unix.ok_or_else(|| BuildError::missing_field("reviewed_at_unix"))?,
            reviewed_by: self.reviewed_by,
            reject_reasons: self.reject_reasons,
            scores_breakdown: self.scores_breakdown,
            rejected_details: self.rejected_details,
            explanation: self.explanation,
        })
    }
}
