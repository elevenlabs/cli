pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WidgetConfigAvatar {
        #[serde(rename = "orb")]
        #[non_exhaustive]
        Orb {
            #[serde(flatten)]
            data: OrbAvatar,
        },

        #[serde(rename = "url")]
        #[non_exhaustive]
        Url {
            #[serde(flatten)]
            data: UrlAvatar,
        },

        #[serde(rename = "image")]
        #[non_exhaustive]
        Image {
            #[serde(flatten)]
            data: ImageAvatar,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WidgetConfigAvatar {
    pub fn orb(data: OrbAvatar) -> Self {
        Self::Orb { data }
    }

    pub fn url(data: UrlAvatar) -> Self {
        Self::Url { data }
    }

    pub fn image(data: ImageAvatar) -> Self {
        Self::Image { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
