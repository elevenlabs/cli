pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Identifier for an integration connection that depends on an auth connection
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DependentIntegrationConnectionIdentifier {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl DependentIntegrationConnectionIdentifier {
    pub fn builder() -> DependentIntegrationConnectionIdentifierBuilder {
        <DependentIntegrationConnectionIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentIntegrationConnectionIdentifierBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl DependentIntegrationConnectionIdentifierBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DependentIntegrationConnectionIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DependentIntegrationConnectionIdentifierBuilder::id)
    /// - [`name`](DependentIntegrationConnectionIdentifierBuilder::name)
    pub fn build(self) -> Result<DependentIntegrationConnectionIdentifier, BuildError> {
        Ok(DependentIntegrationConnectionIdentifier {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
