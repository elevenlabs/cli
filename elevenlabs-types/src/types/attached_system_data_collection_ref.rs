pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttachedSystemDataCollectionRef {
    /// Id of the referenced built-in system data-collection item.
    pub analysis_item_id: String,
    /// Transcript context ('conversation' or 'agent') used when running this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
}

impl AttachedSystemDataCollectionRef {
    pub fn builder() -> AttachedSystemDataCollectionRefBuilder {
        <AttachedSystemDataCollectionRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachedSystemDataCollectionRefBuilder {
    analysis_item_id: Option<String>,
    scope: Option<AnalysisScope>,
}

impl AttachedSystemDataCollectionRefBuilder {
    pub fn analysis_item_id(mut self, value: impl Into<String>) -> Self {
        self.analysis_item_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachedSystemDataCollectionRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis_item_id`](AttachedSystemDataCollectionRefBuilder::analysis_item_id)
    pub fn build(self) -> Result<AttachedSystemDataCollectionRef, BuildError> {
        Ok(AttachedSystemDataCollectionRef {
            analysis_item_id: self.analysis_item_id.ok_or_else(|| BuildError::missing_field("analysis_item_id"))?,
            scope: self.scope,
        })
    }
}
