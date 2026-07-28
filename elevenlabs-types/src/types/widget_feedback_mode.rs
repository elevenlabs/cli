pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WidgetFeedbackMode {
    None,
    During,
    End,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WidgetFeedbackMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::During => serializer.serialize_str("during"),
            Self::End => serializer.serialize_str("end"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetFeedbackMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "during" => Ok(Self::During),
            "end" => Ok(Self::End),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WidgetFeedbackMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::During => write!(f, "during"),
            Self::End => write!(f, "end"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
