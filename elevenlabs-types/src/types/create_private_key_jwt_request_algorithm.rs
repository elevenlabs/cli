pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// JWT signing algorithm
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePrivateKeyJwtRequestAlgorithm {
    Hs256,
    Hs384,
    Hs512,
    Rs256,
    Rs384,
    Rs512,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePrivateKeyJwtRequestAlgorithm {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Hs256 => serializer.serialize_str("HS256"),
            Self::Hs384 => serializer.serialize_str("HS384"),
            Self::Hs512 => serializer.serialize_str("HS512"),
            Self::Rs256 => serializer.serialize_str("RS256"),
            Self::Rs384 => serializer.serialize_str("RS384"),
            Self::Rs512 => serializer.serialize_str("RS512"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePrivateKeyJwtRequestAlgorithm {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "HS256" => Ok(Self::Hs256),
            "HS384" => Ok(Self::Hs384),
            "HS512" => Ok(Self::Hs512),
            "RS256" => Ok(Self::Rs256),
            "RS384" => Ok(Self::Rs384),
            "RS512" => Ok(Self::Rs512),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePrivateKeyJwtRequestAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hs256 => write!(f, "HS256"),
            Self::Hs384 => write!(f, "HS384"),
            Self::Hs512 => write!(f, "HS512"),
            Self::Rs256 => write!(f, "RS256"),
            Self::Rs384 => write!(f, "RS384"),
            Self::Rs512 => write!(f, "RS512"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
