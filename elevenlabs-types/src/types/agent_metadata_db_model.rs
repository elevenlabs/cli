pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentMetadataDbModel {
    #[serde(default)]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    pub updated_at_unix_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from: Option<AgentDefinitionSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<AgentDefinitionSource>,
}

impl AgentMetadataDbModel {
    pub fn builder() -> AgentMetadataDbModelBuilder {
        <AgentMetadataDbModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentMetadataDbModelBuilder {
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
    created_from: Option<AgentDefinitionSource>,
    last_updated_from: Option<AgentDefinitionSource>,
}

impl AgentMetadataDbModelBuilder {
    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    pub fn created_from(mut self, value: AgentDefinitionSource) -> Self {
        self.created_from = Some(value);
        self
    }

    pub fn last_updated_from(mut self, value: AgentDefinitionSource) -> Self {
        self.last_updated_from = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentMetadataDbModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at_unix_secs`](AgentMetadataDbModelBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](AgentMetadataDbModelBuilder::updated_at_unix_secs)
    pub fn build(self) -> Result<AgentMetadataDbModel, BuildError> {
        Ok(AgentMetadataDbModel {
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self.updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
            created_from: self.created_from,
            last_updated_from: self.last_updated_from,
        })
    }
}
