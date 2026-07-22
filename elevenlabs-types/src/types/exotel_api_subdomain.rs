pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExotelApiSubdomain {
    ApiInExotelCom,
    ApiExotelCom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExotelApiSubdomain {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApiInExotelCom => serializer.serialize_str("api.in.exotel.com"),
            Self::ApiExotelCom => serializer.serialize_str("api.exotel.com"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExotelApiSubdomain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api.in.exotel.com" => Ok(Self::ApiInExotelCom),
            "api.exotel.com" => Ok(Self::ApiExotelCom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExotelApiSubdomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiInExotelCom => write!(f, "api.in.exotel.com"),
            Self::ApiExotelCom => write!(f, "api.exotel.com"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
