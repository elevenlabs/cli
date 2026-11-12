pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Single-word processing step shown in the UI status badge.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcessingProgress {
    Converting,
    Storing,
    Scraping,
    Crawling,
    Extracting,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProcessingProgress {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Converting => serializer.serialize_str("Converting"),
            Self::Storing => serializer.serialize_str("Storing"),
            Self::Scraping => serializer.serialize_str("Scraping"),
            Self::Crawling => serializer.serialize_str("Crawling"),
            Self::Extracting => serializer.serialize_str("Extracting"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProcessingProgress {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Converting" => Ok(Self::Converting),
            "Storing" => Ok(Self::Storing),
            "Scraping" => Ok(Self::Scraping),
            "Crawling" => Ok(Self::Crawling),
            "Extracting" => Ok(Self::Extracting),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProcessingProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Converting => write!(f, "Converting"),
            Self::Storing => write!(f, "Storing"),
            Self::Scraping => write!(f, "Scraping"),
            Self::Crawling => write!(f, "Crawling"),
            Self::Extracting => write!(f, "Extracting"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
