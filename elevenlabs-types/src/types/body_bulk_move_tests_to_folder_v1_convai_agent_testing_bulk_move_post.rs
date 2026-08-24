pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePost {
    /// The IDs of tests or folders to move.
    #[serde(default)]
    pub entity_ids: Vec<String>,
    /// The folder to move the entities to. If not set, the entities will be moved to the root folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to: Option<String>,
}

impl BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePost {
    pub fn builder() -> BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePostBuilder {
        <BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePostBuilder {
    entity_ids: Option<Vec<String>>,
    move_to: Option<String>,
}

impl BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePostBuilder {
    pub fn entity_ids(mut self, value: Vec<String>) -> Self {
        self.entity_ids = Some(value);
        self
    }

    pub fn move_to(mut self, value: impl Into<String>) -> Self {
        self.move_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_ids`](BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePostBuilder::entity_ids)
    pub fn build(self) -> Result<BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePost, BuildError> {
        Ok(BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePost {
            entity_ids: self.entity_ids.ok_or_else(|| BuildError::missing_field("entity_ids"))?,
            move_to: self.move_to,
        })
    }
}

