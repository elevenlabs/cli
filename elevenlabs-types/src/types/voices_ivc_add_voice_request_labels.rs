pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AddVoiceRequestLabels {
        StringToStringMap(HashMap<String, String>),

        String(String),
}

impl AddVoiceRequestLabels {
    pub fn is_string_to_string_map(&self) -> bool {
        matches!(self, Self::StringToStringMap(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_string_to_string_map(&self) -> Option<&HashMap<String, String>> {
        match self {
                    Self::StringToStringMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_string_map(self) -> Option<HashMap<String, String>> {
        match self {
                    Self::StringToStringMap(value) => Some(value),
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
