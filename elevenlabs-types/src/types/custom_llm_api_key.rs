pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CustomLlmApiKey {
        ConvAiSecretLocator(ConvAiSecretLocator),

        ConvAiEnvVarLocator(ConvAiEnvVarLocator),
}

impl CustomLlmApiKey {
    pub fn is_conv_ai_secret_locator(&self) -> bool {
        matches!(self, Self::ConvAiSecretLocator(_))
    }

    pub fn is_conv_ai_env_var_locator(&self) -> bool {
        matches!(self, Self::ConvAiEnvVarLocator(_))
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

    pub fn as_conv_ai_env_var_locator(&self) -> Option<&ConvAiEnvVarLocator> {
        match self {
                    Self::ConvAiEnvVarLocator(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conv_ai_env_var_locator(self) -> Option<ConvAiEnvVarLocator> {
        match self {
                    Self::ConvAiEnvVarLocator(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CustomLlmApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConvAiSecretLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConvAiEnvVarLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
