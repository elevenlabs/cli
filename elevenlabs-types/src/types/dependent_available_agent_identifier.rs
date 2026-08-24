pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DependentAvailableAgentIdentifier {
    /// If the agent is a transitive dependent, contains IDs of the resources that the agent depends on directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_resource_ids: Option<Vec<String>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    pub access_level: DependentAvailableAgentIdentifierAccessLevel,
}

impl DependentAvailableAgentIdentifier {
    pub fn builder() -> DependentAvailableAgentIdentifierBuilder {
        <DependentAvailableAgentIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentAvailableAgentIdentifierBuilder {
    referenced_resource_ids: Option<Vec<String>>,
    id: Option<String>,
    name: Option<String>,
    created_at_unix_secs: Option<i64>,
    access_level: Option<DependentAvailableAgentIdentifierAccessLevel>,
}

impl DependentAvailableAgentIdentifierBuilder {
    pub fn referenced_resource_ids(mut self, value: Vec<String>) -> Self {
        self.referenced_resource_ids = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn access_level(mut self, value: DependentAvailableAgentIdentifierAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DependentAvailableAgentIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DependentAvailableAgentIdentifierBuilder::id)
    /// - [`name`](DependentAvailableAgentIdentifierBuilder::name)
    /// - [`created_at_unix_secs`](DependentAvailableAgentIdentifierBuilder::created_at_unix_secs)
    /// - [`access_level`](DependentAvailableAgentIdentifierBuilder::access_level)
    pub fn build(self) -> Result<DependentAvailableAgentIdentifier, BuildError> {
        Ok(DependentAvailableAgentIdentifier {
            referenced_resource_ids: self.referenced_resource_ids,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            access_level: self.access_level.ok_or_else(|| BuildError::missing_field("access_level"))?,
        })
    }
}
