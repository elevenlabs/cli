pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How to attach protocol_discriminator. 'prefix' prepends the octet to the hex payload (User-to-User=XX<hex>;encoding=hex). 'pd_parameter' sends it as a separate parameter (User-to-User=<hex>;pd=XX;encoding=hex). Ignored when protocol_discriminator is unset.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UuiTransferConfigProtocolDiscriminatorMode {
    Prefix,
    PdParameter,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UuiTransferConfigProtocolDiscriminatorMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Prefix => serializer.serialize_str("prefix"),
            Self::PdParameter => serializer.serialize_str("pd_parameter"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UuiTransferConfigProtocolDiscriminatorMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "prefix" => Ok(Self::Prefix),
            "pd_parameter" => Ok(Self::PdParameter),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UuiTransferConfigProtocolDiscriminatorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prefix => write!(f, "prefix"),
            Self::PdParameter => write!(f, "pd_parameter"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
