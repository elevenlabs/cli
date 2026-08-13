pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetricType {
    Credits,
    TtsCharacters,
    MinutesUsed,
    RequestCount,
    TtfbAvg,
    TtfbP95,
    FiatUnitsSpent,
    Concurrency,
    ConcurrencyAverage,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MetricType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Credits => serializer.serialize_str("credits"),
            Self::TtsCharacters => serializer.serialize_str("tts_characters"),
            Self::MinutesUsed => serializer.serialize_str("minutes_used"),
            Self::RequestCount => serializer.serialize_str("request_count"),
            Self::TtfbAvg => serializer.serialize_str("ttfb_avg"),
            Self::TtfbP95 => serializer.serialize_str("ttfb_p95"),
            Self::FiatUnitsSpent => serializer.serialize_str("fiat_units_spent"),
            Self::Concurrency => serializer.serialize_str("concurrency"),
            Self::ConcurrencyAverage => serializer.serialize_str("concurrency_average"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MetricType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "credits" => Ok(Self::Credits),
            "tts_characters" => Ok(Self::TtsCharacters),
            "minutes_used" => Ok(Self::MinutesUsed),
            "request_count" => Ok(Self::RequestCount),
            "ttfb_avg" => Ok(Self::TtfbAvg),
            "ttfb_p95" => Ok(Self::TtfbP95),
            "fiat_units_spent" => Ok(Self::FiatUnitsSpent),
            "concurrency" => Ok(Self::Concurrency),
            "concurrency_average" => Ok(Self::ConcurrencyAverage),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credits => write!(f, "credits"),
            Self::TtsCharacters => write!(f, "tts_characters"),
            Self::MinutesUsed => write!(f, "minutes_used"),
            Self::RequestCount => write!(f, "request_count"),
            Self::TtfbAvg => write!(f, "ttfb_avg"),
            Self::TtfbP95 => write!(f, "ttfb_p95"),
            Self::FiatUnitsSpent => write!(f, "fiat_units_spent"),
            Self::Concurrency => write!(f, "concurrency"),
            Self::ConcurrencyAverage => write!(f, "concurrency_average"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
