pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UrlAvatar {
    /// The custom URL of the avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_url: Option<String>,
}

impl UrlAvatar {
    pub fn builder() -> UrlAvatarBuilder {
        <UrlAvatarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UrlAvatarBuilder {
    custom_url: Option<String>,
}

impl UrlAvatarBuilder {
    pub fn custom_url(mut self, value: impl Into<String>) -> Self {
        self.custom_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UrlAvatar`].
    pub fn build(self) -> Result<UrlAvatar, BuildError> {
        Ok(UrlAvatar {
            custom_url: self.custom_url,
        })
    }
}
