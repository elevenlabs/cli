pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Published versions, newest first; current_version_id is the main that unpinned refs resolve to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisItemVersionsResponseModel {
    #[serde(default)]
    pub versions: Vec<AnalysisItemVersionResponseModel>,
    #[serde(default)]
    pub current_version_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl AnalysisItemVersionsResponseModel {
    pub fn builder() -> AnalysisItemVersionsResponseModelBuilder {
        <AnalysisItemVersionsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemVersionsResponseModelBuilder {
    versions: Option<Vec<AnalysisItemVersionResponseModel>>,
    current_version_id: Option<String>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl AnalysisItemVersionsResponseModelBuilder {
    pub fn versions(mut self, value: Vec<AnalysisItemVersionResponseModel>) -> Self {
        self.versions = Some(value);
        self
    }

    pub fn current_version_id(mut self, value: impl Into<String>) -> Self {
        self.current_version_id = Some(value.into());
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemVersionsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`versions`](AnalysisItemVersionsResponseModelBuilder::versions)
    /// - [`current_version_id`](AnalysisItemVersionsResponseModelBuilder::current_version_id)
    /// - [`has_more`](AnalysisItemVersionsResponseModelBuilder::has_more)
    pub fn build(self) -> Result<AnalysisItemVersionsResponseModel, BuildError> {
        Ok(AnalysisItemVersionsResponseModel {
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
            current_version_id: self.current_version_id.ok_or_else(|| BuildError::missing_field("current_version_id"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
