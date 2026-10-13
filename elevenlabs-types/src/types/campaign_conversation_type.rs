pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignConversationType {
    Initial,
    Retry,
    Callback,
    Redirect,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignConversationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Initial => serializer.serialize_str("initial"),
            Self::Retry => serializer.serialize_str("retry"),
            Self::Callback => serializer.serialize_str("callback"),
            Self::Redirect => serializer.serialize_str("redirect"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignConversationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "initial" => Ok(Self::Initial),
            "retry" => Ok(Self::Retry),
            "callback" => Ok(Self::Callback),
            "redirect" => Ok(Self::Redirect),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignConversationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initial => write!(f, "initial"),
            Self::Retry => write!(f, "retry"),
            Self::Callback => write!(f, "callback"),
            Self::Redirect => write!(f, "redirect"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
