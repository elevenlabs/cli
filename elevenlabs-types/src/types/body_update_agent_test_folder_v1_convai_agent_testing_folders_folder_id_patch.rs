pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatch {
    /// The new name for the folder
    #[serde(default)]
    pub name: String,
}

impl BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatch {
    pub fn builder() -> BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatchBuilder {
        <BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatchBuilder {
    name: Option<String>,
}

impl BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatchBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatchBuilder::name)
    pub fn build(self) -> Result<BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatch, BuildError> {
        Ok(BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatch {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

