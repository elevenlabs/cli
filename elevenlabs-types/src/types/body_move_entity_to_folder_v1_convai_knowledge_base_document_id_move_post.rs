pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePost {
    /// The folder to move the entities to. If not set, the entities will be moved to the root folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to: Option<String>,
}

impl BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePost {
    pub fn builder() -> BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePostBuilder {
        <BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePostBuilder {
    move_to: Option<String>,
}

impl BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePostBuilder {
    pub fn move_to(mut self, value: impl Into<String>) -> Self {
        self.move_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePost`].
    pub fn build(self) -> Result<BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePost, BuildError> {
        Ok(BodyMoveEntityToFolderV1ConvaiKnowledgeBaseDocumentIdMovePost {
            move_to: self.move_to,
        })
    }
}

