pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateVoiceCollectionV1VoicesCollectionsPost {
    /// Title of the collection to create/update.
    #[serde(default)]
    pub title: String,
    /// Icon of the collection to create/update.
    #[serde(default)]
    pub icon: String,
}

impl BodyCreateVoiceCollectionV1VoicesCollectionsPost {
    pub fn builder() -> BodyCreateVoiceCollectionV1VoicesCollectionsPostBuilder {
        <BodyCreateVoiceCollectionV1VoicesCollectionsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateVoiceCollectionV1VoicesCollectionsPostBuilder {
    title: Option<String>,
    icon: Option<String>,
}

impl BodyCreateVoiceCollectionV1VoicesCollectionsPostBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateVoiceCollectionV1VoicesCollectionsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](BodyCreateVoiceCollectionV1VoicesCollectionsPostBuilder::title)
    /// - [`icon`](BodyCreateVoiceCollectionV1VoicesCollectionsPostBuilder::icon)
    pub fn build(self) -> Result<BodyCreateVoiceCollectionV1VoicesCollectionsPost, BuildError> {
        Ok(BodyCreateVoiceCollectionV1VoicesCollectionsPost {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
        })
    }
}

