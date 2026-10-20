pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OpenAiSessionConfigInputTurnDetection {
        OpenAiTurnDetection(OpenAiTurnDetection),

        String(String),
}

impl OpenAiSessionConfigInputTurnDetection {
    pub fn is_open_ai_turn_detection(&self) -> bool {
        matches!(self, Self::OpenAiTurnDetection(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_open_ai_turn_detection(&self) -> Option<&OpenAiTurnDetection> {
        match self {
                    Self::OpenAiTurnDetection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_open_ai_turn_detection(self) -> Option<OpenAiTurnDetection> {
        match self {
                    Self::OpenAiTurnDetection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for OpenAiSessionConfigInputTurnDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenAiTurnDetection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
