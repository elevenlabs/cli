pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OpenAiSessionConfigToolChoice {
        String(String),

        OpenAiToolChoiceFunction(OpenAiToolChoiceFunction),
}

impl OpenAiSessionConfigToolChoice {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_open_ai_tool_choice_function(&self) -> bool {
        matches!(self, Self::OpenAiToolChoiceFunction(_))
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

    pub fn as_open_ai_tool_choice_function(&self) -> Option<&OpenAiToolChoiceFunction> {
        match self {
                    Self::OpenAiToolChoiceFunction(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_open_ai_tool_choice_function(self) -> Option<OpenAiToolChoiceFunction> {
        match self {
                    Self::OpenAiToolChoiceFunction(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for OpenAiSessionConfigToolChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::OpenAiToolChoiceFunction(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
