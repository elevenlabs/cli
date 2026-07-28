pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum McpToolConfigOverrideCreateRequestModelToolCallSound {
        ToolCallSoundType(ToolCallSoundType),

        String(String),
}

impl McpToolConfigOverrideCreateRequestModelToolCallSound {
    pub fn is_tool_call_sound_type(&self) -> bool {
        matches!(self, Self::ToolCallSoundType(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_tool_call_sound_type(&self) -> Option<&ToolCallSoundType> {
        match self {
                    Self::ToolCallSoundType(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tool_call_sound_type(self) -> Option<ToolCallSoundType> {
        match self {
                    Self::ToolCallSoundType(value) => Some(value),
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
