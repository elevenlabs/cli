pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Server chunk containing encoded audio for a specific context and optional alignment metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextToDialogueWebsocketAudioChunkMulti {
    /// Base64-encoded audio bytes for the selected `output_format`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// Present when `sync_alignment` query parameter is `true` and the model returned timing data for the chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<DialogueTextAlignment>,
    /// Reserved for future use; currently unused by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<DialogueTextAlignment>,
    /// The context this audio chunk belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl TextToDialogueWebsocketAudioChunkMulti {
    pub fn builder() -> TextToDialogueWebsocketAudioChunkMultiBuilder {
        <TextToDialogueWebsocketAudioChunkMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketAudioChunkMultiBuilder {
    audio: Option<String>,
    alignment: Option<DialogueTextAlignment>,
    normalized_alignment: Option<DialogueTextAlignment>,
    context_id: Option<String>,
}

impl TextToDialogueWebsocketAudioChunkMultiBuilder {
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

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketAudioChunkMulti`].
    pub fn build(self) -> Result<TextToDialogueWebsocketAudioChunkMulti, BuildError> {
        Ok(TextToDialogueWebsocketAudioChunkMulti {
            audio: self.audio,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
            context_id: self.context_id,
        })
    }
}
