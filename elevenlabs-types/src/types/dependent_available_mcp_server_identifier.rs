pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DependentAvailableMcpServerIdentifier {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    pub access_level: DependentAvailableMcpServerIdentifierAccessLevel,
}

impl DependentAvailableMcpServerIdentifier {
    pub fn builder() -> DependentAvailableMcpServerIdentifierBuilder {
        <DependentAvailableMcpServerIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentAvailableMcpServerIdentifierBuilder {
    id: Option<String>,
    name: Option<String>,
    created_at_unix_secs: Option<i64>,
    access_level: Option<DependentAvailableMcpServerIdentifierAccessLevel>,
}

impl DependentAvailableMcpServerIdentifierBuilder {
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

    pub fn access_level(mut self, value: DependentAvailableMcpServerIdentifierAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DependentAvailableMcpServerIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DependentAvailableMcpServerIdentifierBuilder::id)
    /// - [`name`](DependentAvailableMcpServerIdentifierBuilder::name)
    /// - [`created_at_unix_secs`](DependentAvailableMcpServerIdentifierBuilder::created_at_unix_secs)
    /// - [`access_level`](DependentAvailableMcpServerIdentifierBuilder::access_level)
    pub fn build(self) -> Result<DependentAvailableMcpServerIdentifier, BuildError> {
        Ok(DependentAvailableMcpServerIdentifier {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            access_level: self.access_level.ok_or_else(|| BuildError::missing_field("access_level"))?,
        })
    }
}
