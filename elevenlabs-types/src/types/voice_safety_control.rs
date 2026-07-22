pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceSafetyControl {
    None,
    Ban,
    Captcha,
    EnterpriseBan,
    EnterpriseCaptcha,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceSafetyControl {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("NONE"),
            Self::Ban => serializer.serialize_str("BAN"),
            Self::Captcha => serializer.serialize_str("CAPTCHA"),
            Self::EnterpriseBan => serializer.serialize_str("ENTERPRISE_BAN"),
            Self::EnterpriseCaptcha => serializer.serialize_str("ENTERPRISE_CAPTCHA"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceSafetyControl {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NONE" => Ok(Self::None),
            "BAN" => Ok(Self::Ban),
            "CAPTCHA" => Ok(Self::Captcha),
            "ENTERPRISE_BAN" => Ok(Self::EnterpriseBan),
            "ENTERPRISE_CAPTCHA" => Ok(Self::EnterpriseCaptcha),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceSafetyControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Ban => write!(f, "BAN"),
            Self::Captcha => write!(f, "CAPTCHA"),
            Self::EnterpriseBan => write!(f, "ENTERPRISE_BAN"),
            Self::EnterpriseCaptcha => write!(f, "ENTERPRISE_CAPTCHA"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
