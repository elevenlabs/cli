pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPost {
    /// URL of the page to extract content from.
    #[serde(default)]
    pub url: String,
    /// Author used in the player and inserted at the start of the uploaded article. If not provided, the default author set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Title used in the player and inserted at the top of the uploaded article. If not provided, the default title set in the Player settings is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPost {
    pub fn builder() -> BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPostBuilder {
        <BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPostBuilder {
    url: Option<String>,
    author: Option<String>,
    title: Option<String>,
}

impl BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPostBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPostBuilder::url)
    pub fn build(self) -> Result<BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPost, BuildError> {
        Ok(BodyUpdateAudioNativeContentFromUrlV1AudioNativeContentPost {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            author: self.author,
            title: self.title,
        })
    }
}

