pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeResellerTierRequestBillingPeriod {
    MonthlyPeriod,
    AnnualPeriod,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChangeResellerTierRequestBillingPeriod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MonthlyPeriod => serializer.serialize_str("monthly_period"),
            Self::AnnualPeriod => serializer.serialize_str("annual_period"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChangeResellerTierRequestBillingPeriod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly_period" => Ok(Self::MonthlyPeriod),
            "annual_period" => Ok(Self::AnnualPeriod),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChangeResellerTierRequestBillingPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MonthlyPeriod => write!(f, "monthly_period"),
            Self::AnnualPeriod => write!(f, "annual_period"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
