pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentTestFolderResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// The path from the root folder to the current folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<Vec<AgentTestFolderPathSegmentResponseModel>>,
    /// The number of direct children (tests and subfolders) in this folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i64>,
}

impl GetAgentTestFolderResponseModel {
    pub fn builder() -> GetAgentTestFolderResponseModelBuilder {
        <GetAgentTestFolderResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentTestFolderResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    folder_path: Option<Vec<AgentTestFolderPathSegmentResponseModel>>,
    children_count: Option<i64>,
}

impl GetAgentTestFolderResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn folder_path(mut self, value: Vec<AgentTestFolderPathSegmentResponseModel>) -> Self {
        self.folder_path = Some(value);
        self
    }

    pub fn children_count(mut self, value: i64) -> Self {
        self.children_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentTestFolderResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetAgentTestFolderResponseModelBuilder::id)
    /// - [`name`](GetAgentTestFolderResponseModelBuilder::name)
    pub fn build(self) -> Result<GetAgentTestFolderResponseModel, BuildError> {
        Ok(GetAgentTestFolderResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            folder_path: self.folder_path,
            children_count: self.children_count,
        })
    }
}
