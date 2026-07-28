pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BodyCreatePodcastV1StudioPodcastsPostMode {
        #[serde(rename = "conversation")]
        #[non_exhaustive]
        Conversation {
            #[serde(default)]
            conversation: PodcastConversationModeData,
        },

        #[serde(rename = "bulletin")]
        #[non_exhaustive]
        Bulletin {
            #[serde(default)]
            bulletin: PodcastBulletinModeData,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl BodyCreatePodcastV1StudioPodcastsPostMode {
    pub fn conversation(conversation: PodcastConversationModeData) -> Self {
        Self::Conversation { conversation }
    }

    pub fn bulletin(bulletin: PodcastBulletinModeData) -> Self {
        Self::Bulletin { bulletin }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
