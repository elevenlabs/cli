pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveVoiceFromUserVoiceCollectionResponseModel {
    /// The ID of the voice collection.
    #[serde(default)]
    pub collection_id: String,
    /// The ID of the voice that was removed.
    #[serde(default)]
    pub voice_id: String,
}

impl RemoveVoiceFromUserVoiceCollectionResponseModel {
    pub fn builder() -> RemoveVoiceFromUserVoiceCollectionResponseModelBuilder {
        <RemoveVoiceFromUserVoiceCollectionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveVoiceFromUserVoiceCollectionResponseModelBuilder {
    collection_id: Option<String>,
    voice_id: Option<String>,
}

impl RemoveVoiceFromUserVoiceCollectionResponseModelBuilder {
    pub fn collection_id(mut self, value: impl Into<String>) -> Self {
        self.collection_id = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RemoveVoiceFromUserVoiceCollectionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`collection_id`](RemoveVoiceFromUserVoiceCollectionResponseModelBuilder::collection_id)
    /// - [`voice_id`](RemoveVoiceFromUserVoiceCollectionResponseModelBuilder::voice_id)
    pub fn build(self) -> Result<RemoveVoiceFromUserVoiceCollectionResponseModel, BuildError> {
        Ok(RemoveVoiceFromUserVoiceCollectionResponseModel {
            collection_id: self.collection_id.ok_or_else(|| BuildError::missing_field("collection_id"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
        })
    }
}
