pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OpenAiSessionConfigOutputTracing {
        String(String),

        OpenAiTracingConfiguration(OpenAiTracingConfiguration),
}

impl OpenAiSessionConfigOutputTracing {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_open_ai_tracing_configuration(&self) -> bool {
        matches!(self, Self::OpenAiTracingConfiguration(_))
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

    pub fn as_open_ai_tracing_configuration(&self) -> Option<&OpenAiTracingConfiguration> {
        match self {
                    Self::OpenAiTracingConfiguration(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_open_ai_tracing_configuration(self) -> Option<OpenAiTracingConfiguration> {
        match self {
                    Self::OpenAiTracingConfiguration(value) => Some(value),
                    _ => None,
                }
    }
}
