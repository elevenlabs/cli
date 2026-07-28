pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChapterContentBlockResponseModelNodesItem {
        #[serde(rename = "tts_node")]
        #[non_exhaustive]
        TtsNode {
            #[serde(default)]
            project_voice_ref_id: String,
            #[serde(default)]
            text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            voice_id: Option<String>,
        },

        #[serde(rename = "_other")]
        #[non_exhaustive]
        Other {},

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ChapterContentBlockResponseModelNodesItem {
    pub fn tts_node(project_voice_ref_id: String, text: String) -> Self {
        Self::TtsNode { project_voice_ref_id, text, voice_id: None }
    }

    pub fn other() -> Self {
        Self::Other {}
    }

    pub fn tts_node_with_voice_id(project_voice_ref_id: String, text: String, voice_id: String) -> Self {
        Self::TtsNode { project_voice_ref_id, text, voice_id: Some(voice_id) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
