pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The time interval over which to aggregate the usage data.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsageAggregationInterval {
    Hour,
    Day,
    Week,
    Month,
    Cumulative,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsageAggregationInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Hour => serializer.serialize_str("hour"),
            Self::Day => serializer.serialize_str("day"),
            Self::Week => serializer.serialize_str("week"),
            Self::Month => serializer.serialize_str("month"),
            Self::Cumulative => serializer.serialize_str("cumulative"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsageAggregationInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hour" => Ok(Self::Hour),
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            "cumulative" => Ok(Self::Cumulative),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsageAggregationInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Cumulative => write!(f, "cumulative"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
