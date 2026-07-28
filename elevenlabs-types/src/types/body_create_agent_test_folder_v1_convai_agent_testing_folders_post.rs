pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost {
    /// The name of the folder to create
    #[serde(default)]
    pub name: String,
    /// The ID of the parent folder. If not provided, the folder will be created at the root level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost {
    pub fn builder() -> BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPostBuilder {
        <BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPostBuilder {
    name: Option<String>,
    parent_folder_id: Option<String>,
}

impl BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPostBuilder::name)
    pub fn build(self) -> Result<BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost, BuildError> {
        Ok(BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_folder_id: self.parent_folder_id,
        })
    }
}

