pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrbAvatar {
    /// The first color of the avatar
    #[serde(rename = "color_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color1: Option<String>,
    /// The second color of the avatar
    #[serde(rename = "color_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color2: Option<String>,
}

impl OrbAvatar {
    pub fn builder() -> OrbAvatarBuilder {
        <OrbAvatarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrbAvatarBuilder {
    color1: Option<String>,
    color2: Option<String>,
}

impl OrbAvatarBuilder {
    pub fn color1(mut self, value: impl Into<String>) -> Self {
        self.color1 = Some(value.into());
        self
    }

    pub fn color2(mut self, value: impl Into<String>) -> Self {
        self.color2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrbAvatar`].
    pub fn build(self) -> Result<OrbAvatar, BuildError> {
        Ok(OrbAvatar {
            color1: self.color1,
            color2: self.color2,
        })
    }
}
