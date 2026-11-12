pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Preset tone for an SMS message body.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmsTemplateStyle {
    Friendly,
    Professional,
    Concise,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SmsTemplateStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Friendly => serializer.serialize_str("friendly"),
            Self::Professional => serializer.serialize_str("professional"),
            Self::Concise => serializer.serialize_str("concise"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SmsTemplateStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "friendly" => Ok(Self::Friendly),
            "professional" => Ok(Self::Professional),
            "concise" => Ok(Self::Concise),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SmsTemplateStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Friendly => write!(f, "friendly"),
            Self::Professional => write!(f, "professional"),
            Self::Concise => write!(f, "concise"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
