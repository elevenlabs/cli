pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// LLM model to use for custom guardrail evaluation
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomGuardrailConfigModel {
    Gemini25FlashLite,
    Gemini25Flash,
    Gemini31FlashLite,
    Gemini35Flash,
    ClaudeHaiku45,
    ClaudeSonnet46,
    Gpt54Nano,
    Gpt54Mini,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CustomGuardrailConfigModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Gemini25FlashLite => serializer.serialize_str("gemini-2.5-flash-lite"),
            Self::Gemini25Flash => serializer.serialize_str("gemini-2.5-flash"),
            Self::Gemini31FlashLite => serializer.serialize_str("gemini-3.1-flash-lite"),
            Self::Gemini35Flash => serializer.serialize_str("gemini-3.5-flash"),
            Self::ClaudeHaiku45 => serializer.serialize_str("claude-haiku-4-5"),
            Self::ClaudeSonnet46 => serializer.serialize_str("claude-sonnet-4-6"),
            Self::Gpt54Nano => serializer.serialize_str("gpt-5.4-nano"),
            Self::Gpt54Mini => serializer.serialize_str("gpt-5.4-mini"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CustomGuardrailConfigModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "gemini-2.5-flash-lite" => Ok(Self::Gemini25FlashLite),
            "gemini-2.5-flash" => Ok(Self::Gemini25Flash),
            "gemini-3.1-flash-lite" => Ok(Self::Gemini31FlashLite),
            "gemini-3.5-flash" => Ok(Self::Gemini35Flash),
            "claude-haiku-4-5" => Ok(Self::ClaudeHaiku45),
            "claude-sonnet-4-6" => Ok(Self::ClaudeSonnet46),
            "gpt-5.4-nano" => Ok(Self::Gpt54Nano),
            "gpt-5.4-mini" => Ok(Self::Gpt54Mini),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CustomGuardrailConfigModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gemini25FlashLite => write!(f, "gemini-2.5-flash-lite"),
            Self::Gemini25Flash => write!(f, "gemini-2.5-flash"),
            Self::Gemini31FlashLite => write!(f, "gemini-3.1-flash-lite"),
            Self::Gemini35Flash => write!(f, "gemini-3.5-flash"),
            Self::ClaudeHaiku45 => write!(f, "claude-haiku-4-5"),
            Self::ClaudeSonnet46 => write!(f, "claude-sonnet-4-6"),
            Self::Gpt54Nano => write!(f, "gpt-5.4-nano"),
            Self::Gpt54Mini => write!(f, "gpt-5.4-mini"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
