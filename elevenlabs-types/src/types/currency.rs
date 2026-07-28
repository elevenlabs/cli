pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Currency {
    Usd,
    Eur,
    Inr,
    Pln,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Currency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Usd => serializer.serialize_str("usd"),
            Self::Eur => serializer.serialize_str("eur"),
            Self::Inr => serializer.serialize_str("inr"),
            Self::Pln => serializer.serialize_str("pln"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "usd" => Ok(Self::Usd),
            "eur" => Ok(Self::Eur),
            "inr" => Ok(Self::Inr),
            "pln" => Ok(Self::Pln),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usd => write!(f, "usd"),
            Self::Eur => write!(f, "eur"),
            Self::Inr => write!(f, "inr"),
            Self::Pln => write!(f, "pln"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
