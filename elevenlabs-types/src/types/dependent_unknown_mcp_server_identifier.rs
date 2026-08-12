pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DependentUnknownMcpServerIdentifier {
    #[serde(default)]
    pub id: String,
}

impl DependentUnknownMcpServerIdentifier {
    pub fn builder() -> DependentUnknownMcpServerIdentifierBuilder {
        <DependentUnknownMcpServerIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentUnknownMcpServerIdentifierBuilder {
    id: Option<String>,
}

impl DependentUnknownMcpServerIdentifierBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DependentUnknownMcpServerIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DependentUnknownMcpServerIdentifierBuilder::id)
    pub fn build(self) -> Result<DependentUnknownMcpServerIdentifier, BuildError> {
        Ok(DependentUnknownMcpServerIdentifier {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
