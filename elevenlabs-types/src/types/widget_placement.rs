pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WidgetPlacement {
    TopLeft,
    Top,
    TopRight,
    BottomLeft,
    Bottom,
    BottomRight,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WidgetPlacement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TopLeft => serializer.serialize_str("top-left"),
            Self::Top => serializer.serialize_str("top"),
            Self::TopRight => serializer.serialize_str("top-right"),
            Self::BottomLeft => serializer.serialize_str("bottom-left"),
            Self::Bottom => serializer.serialize_str("bottom"),
            Self::BottomRight => serializer.serialize_str("bottom-right"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetPlacement {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "top-left" => Ok(Self::TopLeft),
            "top" => Ok(Self::Top),
            "top-right" => Ok(Self::TopRight),
            "bottom-left" => Ok(Self::BottomLeft),
            "bottom" => Ok(Self::Bottom),
            "bottom-right" => Ok(Self::BottomRight),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WidgetPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopLeft => write!(f, "top-left"),
            Self::Top => write!(f, "top"),
            Self::TopRight => write!(f, "top-right"),
            Self::BottomLeft => write!(f, "bottom-left"),
            Self::Bottom => write!(f, "bottom"),
            Self::BottomRight => write!(f, "bottom-right"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
