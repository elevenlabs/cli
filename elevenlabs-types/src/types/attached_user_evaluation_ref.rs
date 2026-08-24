pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AttachedUserEvaluationRef {
    /// Id of the referenced user evaluation item.
    #[serde(default)]
    pub analysis_item_id: String,
    /// Primary item version whose result feeds scoring. None tracks the item's latest published version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Extra item versions to also run for comparison (A/B). These are executed and stored but excluded from scoring; the primary version_id is the one that scores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_version_ids: Option<Vec<String>>,
    /// Transcript context ('conversation' or 'agent') used when running this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
    /// Optional relative weight for aggregate scoring.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub weight: Option<f64>,
}

impl AttachedUserEvaluationRef {
    pub fn builder() -> AttachedUserEvaluationRefBuilder {
        <AttachedUserEvaluationRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachedUserEvaluationRefBuilder {
    analysis_item_id: Option<String>,
    version_id: Option<String>,
    additional_version_ids: Option<Vec<String>>,
    scope: Option<AnalysisScope>,
    weight: Option<f64>,
}

impl AttachedUserEvaluationRefBuilder {
    pub fn analysis_item_id(mut self, value: impl Into<String>) -> Self {
        self.analysis_item_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn additional_version_ids(mut self, value: Vec<String>) -> Self {
        self.additional_version_ids = Some(value);
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

    /// Consumes the builder and constructs a [`AttachedUserEvaluationRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis_item_id`](AttachedUserEvaluationRefBuilder::analysis_item_id)
    pub fn build(self) -> Result<AttachedUserEvaluationRef, BuildError> {
        Ok(AttachedUserEvaluationRef {
            analysis_item_id: self.analysis_item_id.ok_or_else(|| BuildError::missing_field("analysis_item_id"))?,
            version_id: self.version_id,
            additional_version_ids: self.additional_version_ids,
            scope: self.scope,
            weight: self.weight,
        })
    }
}
