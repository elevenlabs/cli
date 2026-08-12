pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttachedSystemEvaluationRef {
    /// Id of the referenced built-in system evaluation.
    pub analysis_item_id: AttachedSystemEvaluationRefAnalysisItemId,
    /// Transcript context ('conversation' or 'agent') used when running this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
    /// Optional relative weight for aggregate scoring.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub weight: Option<f64>,
}

impl AttachedSystemEvaluationRef {
    pub fn builder() -> AttachedSystemEvaluationRefBuilder {
        <AttachedSystemEvaluationRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachedSystemEvaluationRefBuilder {
    analysis_item_id: Option<AttachedSystemEvaluationRefAnalysisItemId>,
    scope: Option<AnalysisScope>,
    weight: Option<f64>,
}

impl AttachedSystemEvaluationRefBuilder {
    pub fn analysis_item_id(mut self, value: AttachedSystemEvaluationRefAnalysisItemId) -> Self {
        self.analysis_item_id = Some(value);
        self
    }

    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn weight(mut self, value: f64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachedSystemEvaluationRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis_item_id`](AttachedSystemEvaluationRefBuilder::analysis_item_id)
    pub fn build(self) -> Result<AttachedSystemEvaluationRef, BuildError> {
        Ok(AttachedSystemEvaluationRef {
            analysis_item_id: self.analysis_item_id.ok_or_else(|| BuildError::missing_field("analysis_item_id"))?,
            scope: self.scope,
            weight: self.weight,
        })
    }
}
