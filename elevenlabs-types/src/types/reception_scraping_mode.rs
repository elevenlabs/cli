pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceptionScrapingMode {
    SinglePage,
    DeepScan,
    QuickScan,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceptionScrapingMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SinglePage => serializer.serialize_str("single_page"),
            Self::DeepScan => serializer.serialize_str("deep_scan"),
            Self::QuickScan => serializer.serialize_str("quick_scan"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceptionScrapingMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "single_page" => Ok(Self::SinglePage),
            "deep_scan" => Ok(Self::DeepScan),
            "quick_scan" => Ok(Self::QuickScan),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceptionScrapingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SinglePage => write!(f, "single_page"),
            Self::DeepScan => write!(f, "deep_scan"),
            Self::QuickScan => write!(f, "quick_scan"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
