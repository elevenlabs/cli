pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttachedUserDataCollectionRef {
    /// Id of the referenced user data-collection item.
    #[serde(default)]
    pub analysis_item_id: String,
    /// Pinned item version. None tracks the item's latest published version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Transcript context ('conversation' or 'agent') used when running this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
}

impl AttachedUserDataCollectionRef {
    pub fn builder() -> AttachedUserDataCollectionRefBuilder {
        <AttachedUserDataCollectionRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachedUserDataCollectionRefBuilder {
    analysis_item_id: Option<String>,
    version_id: Option<String>,
    scope: Option<AnalysisScope>,
}

impl AttachedUserDataCollectionRefBuilder {
    pub fn analysis_item_id(mut self, value: impl Into<String>) -> Self {
        self.analysis_item_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachedUserDataCollectionRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis_item_id`](AttachedUserDataCollectionRefBuilder::analysis_item_id)
    pub fn build(self) -> Result<AttachedUserDataCollectionRef, BuildError> {
        Ok(AttachedUserDataCollectionRef {
            analysis_item_id: self.analysis_item_id.ok_or_else(|| BuildError::missing_field("analysis_item_id"))?,
            version_id: self.version_id,
            scope: self.scope,
        })
    }
}
