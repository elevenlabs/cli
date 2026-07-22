pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextToDialogueWebsocketFinalAudioForTurnMulti {
    /// Indicates that the final audio for a given turn of this context has been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final_audio_for_turn: Option<bool>,
    /// The context whose turn has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl TextToDialogueWebsocketFinalAudioForTurnMulti {
    pub fn builder() -> TextToDialogueWebsocketFinalAudioForTurnMultiBuilder {
        <TextToDialogueWebsocketFinalAudioForTurnMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketFinalAudioForTurnMultiBuilder {
    is_final_audio_for_turn: Option<bool>,
    context_id: Option<String>,
}

impl TextToDialogueWebsocketFinalAudioForTurnMultiBuilder {
    pub fn is_final_audio_for_turn(mut self, value: bool) -> Self {
        self.is_final_audio_for_turn = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketFinalAudioForTurnMulti`].
    pub fn build(self) -> Result<TextToDialogueWebsocketFinalAudioForTurnMulti, BuildError> {
        Ok(TextToDialogueWebsocketFinalAudioForTurnMulti {
            is_final_audio_for_turn: self.is_final_audio_for_turn,
            context_id: self.context_id,
        })
    }
}
