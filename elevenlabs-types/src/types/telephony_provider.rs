pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TelephonyProvider {
    Twilio,
    SipTrunk,
    Exotel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TelephonyProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Twilio => serializer.serialize_str("twilio"),
            Self::SipTrunk => serializer.serialize_str("sip_trunk"),
            Self::Exotel => serializer.serialize_str("exotel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TelephonyProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "twilio" => Ok(Self::Twilio),
            "sip_trunk" => Ok(Self::SipTrunk),
            "exotel" => Ok(Self::Exotel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TelephonyProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Twilio => write!(f, "twilio"),
            Self::SipTrunk => write!(f, "sip_trunk"),
            Self::Exotel => write!(f, "exotel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
