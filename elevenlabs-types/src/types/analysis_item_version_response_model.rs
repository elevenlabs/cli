pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisItemVersionResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub version_number: i64,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    pub definition: AnalysisItemVersionResponseModelDefinition,
}

impl AnalysisItemVersionResponseModel {
    pub fn builder() -> AnalysisItemVersionResponseModelBuilder {
        <AnalysisItemVersionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemVersionResponseModelBuilder {
    id: Option<String>,
    version_number: Option<i64>,
    created_at_unix_secs: Option<i64>,
    definition: Option<AnalysisItemVersionResponseModelDefinition>,
}

impl AnalysisItemVersionResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn version_number(mut self, value: i64) -> Self {
        self.version_number = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn definition(mut self, value: AnalysisItemVersionResponseModelDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemVersionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnalysisItemVersionResponseModelBuilder::id)
    /// - [`version_number`](AnalysisItemVersionResponseModelBuilder::version_number)
    /// - [`created_at_unix_secs`](AnalysisItemVersionResponseModelBuilder::created_at_unix_secs)
    /// - [`definition`](AnalysisItemVersionResponseModelBuilder::definition)
    pub fn build(self) -> Result<AnalysisItemVersionResponseModel, BuildError> {
        Ok(AnalysisItemVersionResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version_number: self.version_number.ok_or_else(|| BuildError::missing_field("version_number"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            definition: self.definition.ok_or_else(|| BuildError::missing_field("definition"))?,
        })
    }
}
