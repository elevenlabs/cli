pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmbeddingModelEnum {
    E5Mistral7BInstruct,
    MultilingualE5LargeInstruct,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmbeddingModelEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::E5Mistral7BInstruct => serializer.serialize_str("e5_mistral_7b_instruct"),
            Self::MultilingualE5LargeInstruct => serializer.serialize_str("multilingual_e5_large_instruct"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmbeddingModelEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "e5_mistral_7b_instruct" => Ok(Self::E5Mistral7BInstruct),
            "multilingual_e5_large_instruct" => Ok(Self::MultilingualE5LargeInstruct),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmbeddingModelEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::E5Mistral7BInstruct => write!(f, "e5_mistral_7b_instruct"),
            Self::MultilingualE5LargeInstruct => write!(f, "multilingual_e5_large_instruct"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
