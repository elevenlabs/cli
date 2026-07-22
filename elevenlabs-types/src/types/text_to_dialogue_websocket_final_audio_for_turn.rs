pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TextToDialogueWebsocketFinalAudioForTurn {
    /// Indicates that the final audio for a given turn has been sent.
    pub is_final_audio_for_turn: Option<bool>,
}

impl TextToDialogueWebsocketFinalAudioForTurn {
    pub fn builder() -> TextToDialogueWebsocketFinalAudioForTurnBuilder {
        <TextToDialogueWebsocketFinalAudioForTurnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketFinalAudioForTurnBuilder {
    is_final_audio_for_turn: Option<bool>,
}

impl TextToDialogueWebsocketFinalAudioForTurnBuilder {
    pub fn is_final_audio_for_turn(mut self, value: bool) -> Self {
        self.is_final_audio_for_turn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketFinalAudioForTurn`].
    pub fn build(self) -> Result<TextToDialogueWebsocketFinalAudioForTurn, BuildError> {
        Ok(TextToDialogueWebsocketFinalAudioForTurn {
            is_final_audio_for_turn: self.is_final_audio_for_turn,
        })
    }
}
