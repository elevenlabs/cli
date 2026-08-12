pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum McpServerConfigInputUrl {
        String(String),

        ConvAiSecretLocator(ConvAiSecretLocator),
}

impl McpServerConfigInputUrl {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_conv_ai_secret_locator(&self) -> bool {
        matches!(self, Self::ConvAiSecretLocator(_))
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

    pub fn as_conv_ai_secret_locator(&self) -> Option<&ConvAiSecretLocator> {
        match self {
                    Self::ConvAiSecretLocator(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conv_ai_secret_locator(self) -> Option<ConvAiSecretLocator> {
        match self {
                    Self::ConvAiSecretLocator(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for McpServerConfigInputUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::ConvAiSecretLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
