pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteUserVoiceCollectionResponseModel {
    /// The ID of the deleted voice collection.
    #[serde(default)]
    pub collection_id: String,
}

impl DeleteUserVoiceCollectionResponseModel {
    pub fn builder() -> DeleteUserVoiceCollectionResponseModelBuilder {
        <DeleteUserVoiceCollectionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteUserVoiceCollectionResponseModelBuilder {
    collection_id: Option<String>,
}

impl DeleteUserVoiceCollectionResponseModelBuilder {
    pub fn collection_id(mut self, value: impl Into<String>) -> Self {
        self.collection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteUserVoiceCollectionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`collection_id`](DeleteUserVoiceCollectionResponseModelBuilder::collection_id)
    pub fn build(self) -> Result<DeleteUserVoiceCollectionResponseModel, BuildError> {
        Ok(DeleteUserVoiceCollectionResponseModel {
            collection_id: self.collection_id.ok_or_else(|| BuildError::missing_field("collection_id"))?,
        })
    }
}
