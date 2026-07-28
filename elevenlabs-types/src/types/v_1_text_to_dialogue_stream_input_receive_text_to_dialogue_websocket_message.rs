pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveTextToDialogueWebsocketMessage {
        TextToDialogueWebsocketAudioChunk(TextToDialogueWebsocketAudioChunk),

        TextToDialogueWebsocketFinalAudioForTurn(TextToDialogueWebsocketFinalAudioForTurn),

        TextToDialogueWebsocketFinal(TextToDialogueWebsocketFinal),

        TextToDialogueWebsocketError(TextToDialogueWebsocketError),
}

impl ReceiveTextToDialogueWebsocketMessage {
    pub fn is_text_to_dialogue_websocket_audio_chunk(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketAudioChunk(_))
    }

    pub fn is_text_to_dialogue_websocket_final_audio_for_turn(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketFinalAudioForTurn(_))
    }

    pub fn is_text_to_dialogue_websocket_final(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketFinal(_))
    }

    pub fn is_text_to_dialogue_websocket_error(&self) -> bool {
        matches!(self, Self::TextToDialogueWebsocketError(_))
    }


    pub fn as_text_to_dialogue_websocket_audio_chunk(&self) -> Option<&TextToDialogueWebsocketAudioChunk> {
        match self {
                    Self::TextToDialogueWebsocketAudioChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_audio_chunk(self) -> Option<TextToDialogueWebsocketAudioChunk> {
        match self {
                    Self::TextToDialogueWebsocketAudioChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_text_to_dialogue_websocket_final_audio_for_turn(&self) -> Option<&TextToDialogueWebsocketFinalAudioForTurn> {
        match self {
                    Self::TextToDialogueWebsocketFinalAudioForTurn(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_final_audio_for_turn(self) -> Option<TextToDialogueWebsocketFinalAudioForTurn> {
        match self {
                    Self::TextToDialogueWebsocketFinalAudioForTurn(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_text_to_dialogue_websocket_final(&self) -> Option<&TextToDialogueWebsocketFinal> {
        match self {
                    Self::TextToDialogueWebsocketFinal(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_text_to_dialogue_websocket_final(self) -> Option<TextToDialogueWebsocketFinal> {
        match self {
                    Self::TextToDialogueWebsocketFinal(value) => Some(value),
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

impl fmt::Display for ReceiveTextToDialogueWebsocketMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextToDialogueWebsocketAudioChunk(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TextToDialogueWebsocketFinalAudioForTurn(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TextToDialogueWebsocketFinal(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TextToDialogueWebsocketError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
