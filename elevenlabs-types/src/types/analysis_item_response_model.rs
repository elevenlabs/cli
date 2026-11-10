pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisItemResponseModel {
    #[serde(default)]
    pub id: String,
    pub definition: AnalysisItemResponseModelDefinition,
    pub access_info: ResourceAccessInfo,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    pub updated_at_unix_secs: i64,
    #[serde(default)]
    pub current_version_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_source_id: Option<String>,
}

impl AnalysisItemResponseModel {
    pub fn builder() -> AnalysisItemResponseModelBuilder {
        <AnalysisItemResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemResponseModelBuilder {
    id: Option<String>,
    definition: Option<AnalysisItemResponseModelDefinition>,
    access_info: Option<ResourceAccessInfo>,
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
    current_version_id: Option<String>,
    legacy_source_id: Option<String>,
}

impl AnalysisItemResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn definition(mut self, value: AnalysisItemResponseModelDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    pub fn current_version_id(mut self, value: impl Into<String>) -> Self {
        self.current_version_id = Some(value.into());
        self
    }

    pub fn legacy_source_id(mut self, value: impl Into<String>) -> Self {
        self.legacy_source_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnalysisItemResponseModelBuilder::id)
    /// - [`definition`](AnalysisItemResponseModelBuilder::definition)
    /// - [`access_info`](AnalysisItemResponseModelBuilder::access_info)
    /// - [`created_at_unix_secs`](AnalysisItemResponseModelBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](AnalysisItemResponseModelBuilder::updated_at_unix_secs)
    /// - [`current_version_id`](AnalysisItemResponseModelBuilder::current_version_id)
    pub fn build(self) -> Result<AnalysisItemResponseModel, BuildError> {
        Ok(AnalysisItemResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            definition: self.definition.ok_or_else(|| BuildError::missing_field("definition"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self.updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
            current_version_id: self.current_version_id.ok_or_else(|| BuildError::missing_field("current_version_id"))?,
            legacy_source_id: self.legacy_source_id,
        })
    }
}
