pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageAvatar {
    /// The URL of the avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ImageAvatar {
    pub fn builder() -> ImageAvatarBuilder {
        <ImageAvatarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageAvatarBuilder {
    url: Option<String>,
}

impl ImageAvatarBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImageAvatar`].
    pub fn build(self) -> Result<ImageAvatar, BuildError> {
        Ok(ImageAvatar {
            url: self.url,
        })
    }
}
