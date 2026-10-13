pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatch {
    /// Title of the collection to create/update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Icon of the collection to create/update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatch {
    pub fn builder() -> BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatchBuilder {
        <BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatchBuilder {
    title: Option<String>,
    icon: Option<String>,
}

impl BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatchBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatch`].
    pub fn build(self) -> Result<BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatch, BuildError> {
        Ok(BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatch {
            title: self.title,
            icon: self.icon,
        })
    }
}

