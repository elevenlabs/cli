pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhatsAppTemplateParameterFormat {
    Positional,
    Named,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WhatsAppTemplateParameterFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Positional => serializer.serialize_str("POSITIONAL"),
            Self::Named => serializer.serialize_str("NAMED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WhatsAppTemplateParameterFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "POSITIONAL" => Ok(Self::Positional),
            "NAMED" => Ok(Self::Named),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WhatsAppTemplateParameterFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positional => write!(f, "POSITIONAL"),
            Self::Named => write!(f, "NAMED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
