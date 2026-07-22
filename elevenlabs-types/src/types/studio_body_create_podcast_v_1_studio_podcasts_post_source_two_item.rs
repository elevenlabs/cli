pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BodyCreatePodcastV1StudioPodcastsPostSourceTwoItem {
        #[serde(rename = "text")]
        #[non_exhaustive]
        Text {
            #[serde(flatten)]
            data: PodcastTextSource,
        },

        #[serde(rename = "url")]
        #[non_exhaustive]
        Url {
            #[serde(flatten)]
            data: PodcastUrlSource,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl BodyCreatePodcastV1StudioPodcastsPostSourceTwoItem {
    pub fn text(data: PodcastTextSource) -> Self {
        Self::Text { data }
    }

    pub fn url(data: PodcastUrlSource) -> Self {
        Self::Url { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
