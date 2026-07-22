pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentMetadataResponseModel {
    /// The creation time of the agent in unix seconds
    #[serde(default)]
    pub created_at_unix_secs: i64,
    /// The last update time of the agent in unix seconds
    #[serde(default)]
    pub updated_at_unix_secs: i64,
}

impl AgentMetadataResponseModel {
    pub fn builder() -> AgentMetadataResponseModelBuilder {
        <AgentMetadataResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentMetadataResponseModelBuilder {
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
}

impl AgentMetadataResponseModelBuilder {
    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentMetadataResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at_unix_secs`](AgentMetadataResponseModelBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](AgentMetadataResponseModelBuilder::updated_at_unix_secs)
    pub fn build(self) -> Result<AgentMetadataResponseModel, BuildError> {
        Ok(AgentMetadataResponseModel {
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self.updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
        })
    }
}
