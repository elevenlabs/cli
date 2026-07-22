pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A model that represents an agent dependent on a knowledge base/tools
/// to which the user has no direct access.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DependentUnknownAgentIdentifier {
    /// If the agent is a transitive dependent, contains IDs of the resources that the agent depends on directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_resource_ids: Option<Vec<String>>,
    #[serde(default)]
    pub id: String,
}

impl DependentUnknownAgentIdentifier {
    pub fn builder() -> DependentUnknownAgentIdentifierBuilder {
        <DependentUnknownAgentIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentUnknownAgentIdentifierBuilder {
    referenced_resource_ids: Option<Vec<String>>,
    id: Option<String>,
}

impl DependentUnknownAgentIdentifierBuilder {
    pub fn referenced_resource_ids(mut self, value: Vec<String>) -> Self {
        self.referenced_resource_ids = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DependentUnknownAgentIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DependentUnknownAgentIdentifierBuilder::id)
    pub fn build(self) -> Result<DependentUnknownAgentIdentifier, BuildError> {
        Ok(DependentUnknownAgentIdentifier {
            referenced_resource_ids: self.referenced_resource_ids,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
