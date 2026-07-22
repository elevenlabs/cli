pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BillingPeriod {
    MonthlyPeriod,
    ThreeMonthPeriod,
    SixMonthPeriod,
    AnnualPeriod,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BillingPeriod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MonthlyPeriod => serializer.serialize_str("monthly_period"),
            Self::ThreeMonthPeriod => serializer.serialize_str("3_month_period"),
            Self::SixMonthPeriod => serializer.serialize_str("6_month_period"),
            Self::AnnualPeriod => serializer.serialize_str("annual_period"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BillingPeriod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly_period" => Ok(Self::MonthlyPeriod),
            "3_month_period" => Ok(Self::ThreeMonthPeriod),
            "6_month_period" => Ok(Self::SixMonthPeriod),
            "annual_period" => Ok(Self::AnnualPeriod),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BillingPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MonthlyPeriod => write!(f, "monthly_period"),
            Self::ThreeMonthPeriod => write!(f, "3_month_period"),
            Self::SixMonthPeriod => write!(f, "6_month_period"),
            Self::AnnualPeriod => write!(f, "annual_period"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
