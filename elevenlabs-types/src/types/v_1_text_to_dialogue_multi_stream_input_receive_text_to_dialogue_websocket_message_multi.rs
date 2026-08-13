pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveTextToDialogueWebsocketMessageMulti {
        TextToDialogueWebsocketAudioChunkMulti(TextToDialogueWebsocketAudioChunkMulti),

        TextToDialogueWebsocketFinalAudioForTurnMulti(TextToDialogueWebsocketFinalAudioForTurnMulti),

        TextToDialogueWebsocketFinalMulti(TextToDialogueWebsocketFinalMulti),

        TextToDialogueWebsocketError(TextToDialogueWebsocketError),
}

impl ReceiveTextToDialogueWebsocketMessageMulti {
    pub fn is_text_to_dialogue_websocket_audio_chunk_multi(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketAudioChunkMulti(_))
    }

    pub fn is_text_to_dialogue_websocket_final_audio_for_turn_multi(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketFinalAudioForTurnMulti(_))
    }

    pub fn is_text_to_dialogue_websocket_final_multi(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketFinalMulti(_))
    }

    pub fn is_text_to_dialogue_websocket_error(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketError(_))
    }


    pub fn as_text_to_dialogue_websocket_audio_chunk_multi(&self) -> Option<&TextToDialogueWebsocketAudioChunkMulti> {
        match self {
                    Self::TextToDialogueWebsocketAudioChunkMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_audio_chunk_multi(self) -> Option<TextToDialogueWebsocketAudioChunkMulti> {
        match self {
                    Self::TextToDialogueWebsocketAudioChunkMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_text_to_dialogue_websocket_final_audio_for_turn_multi(&self) -> Option<&TextToDialogueWebsocketFinalAudioForTurnMulti> {
        match self {
                    Self::TextToDialogueWebsocketFinalAudioForTurnMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_final_audio_for_turn_multi(self) -> Option<TextToDialogueWebsocketFinalAudioForTurnMulti> {
        match self {
                    Self::TextToDialogueWebsocketFinalAudioForTurnMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_text_to_dialogue_websocket_final_multi(&self) -> Option<&TextToDialogueWebsocketFinalMulti> {
        match self {
                    Self::TextToDialogueWebsocketFinalMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_final_multi(self) -> Option<TextToDialogueWebsocketFinalMulti> {
        match self {
                    Self::TextToDialogueWebsocketFinalMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_text_to_dialogue_websocket_error(&self) -> Option<&TextToDialogueWebsocketError> {
        match self {
                    Self::TextToDialogueWebsocketError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_error(self) -> Option<TextToDialogueWebsocketError> {
        match self {
                    Self::TextToDialogueWebsocketError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveTextToDialogueWebsocketMessageMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextToDialogueWebsocketAudioChunkMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TextToDialogueWebsocketFinalAudioForTurnMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TextToDialogueWebsocketFinalMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TextToDialogueWebsocketError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
