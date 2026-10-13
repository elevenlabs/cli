pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUserVoiceCollectionsResponseModel {
    /// The voice collections the caller can access, sorted by title.
    #[serde(default)]
    pub collections: Vec<UserVoiceCollectionResponseModel>,
}

impl GetUserVoiceCollectionsResponseModel {
    pub fn builder() -> GetUserVoiceCollectionsResponseModelBuilder {
        <GetUserVoiceCollectionsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUserVoiceCollectionsResponseModelBuilder {
    collections: Option<Vec<UserVoiceCollectionResponseModel>>,
}

impl GetUserVoiceCollectionsResponseModelBuilder {
    pub fn collections(mut self, value: Vec<UserVoiceCollectionResponseModel>) -> Self {
        self.collections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetUserVoiceCollectionsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`collections`](GetUserVoiceCollectionsResponseModelBuilder::collections)
    pub fn build(self) -> Result<GetUserVoiceCollectionsResponseModel, BuildError> {
        Ok(GetUserVoiceCollectionsResponseModel {
            collections: self.collections.ok_or_else(|| BuildError::missing_field("collections"))?,
        })
    }
}
