pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WidgetExpandable {
    Never,
    Mobile,
    Desktop,
    Always,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WidgetExpandable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Never => serializer.serialize_str("never"),
            Self::Mobile => serializer.serialize_str("mobile"),
            Self::Desktop => serializer.serialize_str("desktop"),
            Self::Always => serializer.serialize_str("always"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetExpandable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "never" => Ok(Self::Never),
            "mobile" => Ok(Self::Mobile),
            "desktop" => Ok(Self::Desktop),
            "always" => Ok(Self::Always),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WidgetExpandable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Never => write!(f, "never"),
            Self::Mobile => write!(f, "mobile"),
            Self::Desktop => write!(f, "desktop"),
            Self::Always => write!(f, "always"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
