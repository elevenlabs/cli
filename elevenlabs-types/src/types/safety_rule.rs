pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SafetyRule {
    SexualMinors,
    ForgetModeration,
    Extremism,
    ScamFraud,
    Political,
    SelfHarm,
    IllegalDistributionMedical,
    SexualAdults,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SafetyRule {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SexualMinors => serializer.serialize_str("sexual_minors"),
            Self::ForgetModeration => serializer.serialize_str("forget_moderation"),
            Self::Extremism => serializer.serialize_str("extremism"),
            Self::ScamFraud => serializer.serialize_str("scam_fraud"),
            Self::Political => serializer.serialize_str("political"),
            Self::SelfHarm => serializer.serialize_str("self_harm"),
            Self::IllegalDistributionMedical => serializer.serialize_str("illegal_distribution_medical"),
            Self::SexualAdults => serializer.serialize_str("sexual_adults"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SafetyRule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sexual_minors" => Ok(Self::SexualMinors),
            "forget_moderation" => Ok(Self::ForgetModeration),
            "extremism" => Ok(Self::Extremism),
            "scam_fraud" => Ok(Self::ScamFraud),
            "political" => Ok(Self::Political),
            "self_harm" => Ok(Self::SelfHarm),
            "illegal_distribution_medical" => Ok(Self::IllegalDistributionMedical),
            "sexual_adults" => Ok(Self::SexualAdults),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SafetyRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SexualMinors => write!(f, "sexual_minors"),
            Self::ForgetModeration => write!(f, "forget_moderation"),
            Self::Extremism => write!(f, "extremism"),
            Self::ScamFraud => write!(f, "scam_fraud"),
            Self::Political => write!(f, "political"),
            Self::SelfHarm => write!(f, "self_harm"),
            Self::IllegalDistributionMedical => write!(f, "illegal_distribution_medical"),
            Self::SexualAdults => write!(f, "sexual_adults"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
