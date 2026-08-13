pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of resource.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReaderResourceResponseModelResourceType {
    Read,
    Collection,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReaderResourceResponseModelResourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Read => serializer.serialize_str("read"),
            Self::Collection => serializer.serialize_str("collection"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReaderResourceResponseModelResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "read" => Ok(Self::Read),
            "collection" => Ok(Self::Collection),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReaderResourceResponseModelResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Collection => write!(f, "collection"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
