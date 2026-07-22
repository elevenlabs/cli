pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationTokenPurpose {
    SignedUrl,
    ShareableLink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationTokenPurpose {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SignedUrl => serializer.serialize_str("signed_url"),
            Self::ShareableLink => serializer.serialize_str("shareable_link"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationTokenPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "signed_url" => Ok(Self::SignedUrl),
            "shareable_link" => Ok(Self::ShareableLink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationTokenPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignedUrl => write!(f, "signed_url"),
            Self::ShareableLink => write!(f, "shareable_link"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
