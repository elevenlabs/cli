pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum McpServerConfigInputRequestMetaValue {
        String(String),

        Integer(i64),

        Double(f64),

        Boolean(bool),

        ConvAiSecretLocator(ConvAiSecretLocator),

        ConvAiDynamicVariable(ConvAiDynamicVariable),

        ConvAiEnvVarLocator(ConvAiEnvVarLocator),
}

impl McpServerConfigInputRequestMetaValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_conv_ai_secret_locator(&self) -> bool {
        matches!(self, Self::ConvAiSecretLocator(_))
    }

    pub fn is_conv_ai_dynamic_variable(&self) -> bool {
        matches!(self, Self::ConvAiDynamicVariable(_))
    }

    pub fn is_conv_ai_env_var_locator(&self) -> bool {
        matches!(self, Self::ConvAiEnvVarLocator(_))
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

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
                    Self::Boolean(value) => Some(value),
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

    pub fn as_conv_ai_dynamic_variable(&self) -> Option<&ConvAiDynamicVariable> {
        match self {
                    Self::ConvAiDynamicVariable(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conv_ai_dynamic_variable(self) -> Option<ConvAiDynamicVariable> {
        match self {
                    Self::ConvAiDynamicVariable(value) => Some(value),
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

impl fmt::Display for McpServerConfigInputRequestMetaValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Double(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::ConvAiSecretLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConvAiDynamicVariable(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConvAiEnvVarLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
