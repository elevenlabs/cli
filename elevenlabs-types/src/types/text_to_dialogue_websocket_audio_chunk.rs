pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Server chunk containing encoded audio and optional alignment metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextToDialogueWebsocketAudioChunk {
    /// Base64-encoded audio bytes for the selected `output_format`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// Present when `sync_alignment` query parameter is `true` and the model returned timing data for the chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<DialogueTextAlignment>,
    /// Reserved for future use; currently unused by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<DialogueTextAlignment>,
}

impl TextToDialogueWebsocketAudioChunk {
    pub fn builder() -> TextToDialogueWebsocketAudioChunkBuilder {
        <TextToDialogueWebsocketAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketAudioChunkBuilder {
    audio: Option<String>,
    alignment: Option<DialogueTextAlignment>,
    normalized_alignment: Option<DialogueTextAlignment>,
}

impl TextToDialogueWebsocketAudioChunkBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn alignment(mut self, value: DialogueTextAlignment) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: DialogueTextAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketAudioChunk`].
    pub fn build(self) -> Result<TextToDialogueWebsocketAudioChunk, BuildError> {
        Ok(TextToDialogueWebsocketAudioChunk {
            audio: self.audio,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
        })
    }
}
