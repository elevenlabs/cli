pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CrawlType {
    Discovery,
    Sitemap,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CrawlType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Discovery => serializer.serialize_str("discovery"),
            Self::Sitemap => serializer.serialize_str("sitemap"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CrawlType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "discovery" => Ok(Self::Discovery),
            "sitemap" => Ok(Self::Sitemap),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CrawlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discovery => write!(f, "discovery"),
            Self::Sitemap => write!(f, "sitemap"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
