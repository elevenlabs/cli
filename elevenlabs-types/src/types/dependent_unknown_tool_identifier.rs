pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A model that represents an tool dependent on a knowledge base/tools
/// to which the user has no direct access.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DependentUnknownToolIdentifier {
    #[serde(default)]
    pub id: String,
}

impl DependentUnknownToolIdentifier {
    pub fn builder() -> DependentUnknownToolIdentifierBuilder {
        <DependentUnknownToolIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentUnknownToolIdentifierBuilder {
    id: Option<String>,
}

impl DependentUnknownToolIdentifierBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DependentUnknownToolIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DependentUnknownToolIdentifierBuilder::id)
    pub fn build(self) -> Result<DependentUnknownToolIdentifier, BuildError> {
        Ok(DependentUnknownToolIdentifier {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
