pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePost {
    /// The ids of documents or folders from the knowledge base.
    #[serde(default)]
    pub document_ids: Vec<String>,
    /// The folder to move the entities to. If not set, the entities will be moved to the root folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to: Option<String>,
}

impl BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePost {
    pub fn builder() -> BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePostBuilder {
        <BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePostBuilder {
    document_ids: Option<Vec<String>>,
    move_to: Option<String>,
}

impl BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePostBuilder {
    pub fn document_ids(mut self, value: Vec<String>) -> Self {
        self.document_ids = Some(value);
        self
    }

    pub fn move_to(mut self, value: impl Into<String>) -> Self {
        self.move_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePostBuilder::document_ids)
    pub fn build(self) -> Result<BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePost, BuildError> {
        Ok(BodyBulkMoveEntitiesToFolderV1ConvaiKnowledgeBaseBulkMovePost {
            document_ids: self.document_ids.ok_or_else(|| BuildError::missing_field("document_ids"))?,
            move_to: self.move_to,
        })
    }
}

