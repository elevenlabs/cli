pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenesysRegion {
    UsEast1,
    EuWest1,
    ApSoutheast2,
    ApNortheast1,
    EuCentral1,
    UsWest2,
    CaCentral1,
    ApNortheast2,
    EuWest2,
    ApSouth1,
    UsEast2,
    SaEast1,
    MeCentral1,
    ApNortheast3,
    EuCentral2,
    MxCentral1,
    ApSoutheast1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GenesysRegion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UsEast1 => serializer.serialize_str("us_east_1"),
            Self::EuWest1 => serializer.serialize_str("eu_west_1"),
            Self::ApSoutheast2 => serializer.serialize_str("ap_southeast_2"),
            Self::ApNortheast1 => serializer.serialize_str("ap_northeast_1"),
            Self::EuCentral1 => serializer.serialize_str("eu_central_1"),
            Self::UsWest2 => serializer.serialize_str("us_west_2"),
            Self::CaCentral1 => serializer.serialize_str("ca_central_1"),
            Self::ApNortheast2 => serializer.serialize_str("ap_northeast_2"),
            Self::EuWest2 => serializer.serialize_str("eu_west_2"),
            Self::ApSouth1 => serializer.serialize_str("ap_south_1"),
            Self::UsEast2 => serializer.serialize_str("us_east_2"),
            Self::SaEast1 => serializer.serialize_str("sa_east_1"),
            Self::MeCentral1 => serializer.serialize_str("me_central_1"),
            Self::ApNortheast3 => serializer.serialize_str("ap_northeast_3"),
            Self::EuCentral2 => serializer.serialize_str("eu_central_2"),
            Self::MxCentral1 => serializer.serialize_str("mx_central_1"),
            Self::ApSoutheast1 => serializer.serialize_str("ap_southeast_1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GenesysRegion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "us_east_1" => Ok(Self::UsEast1),
            "eu_west_1" => Ok(Self::EuWest1),
            "ap_southeast_2" => Ok(Self::ApSoutheast2),
            "ap_northeast_1" => Ok(Self::ApNortheast1),
            "eu_central_1" => Ok(Self::EuCentral1),
            "us_west_2" => Ok(Self::UsWest2),
            "ca_central_1" => Ok(Self::CaCentral1),
            "ap_northeast_2" => Ok(Self::ApNortheast2),
            "eu_west_2" => Ok(Self::EuWest2),
            "ap_south_1" => Ok(Self::ApSouth1),
            "us_east_2" => Ok(Self::UsEast2),
            "sa_east_1" => Ok(Self::SaEast1),
            "me_central_1" => Ok(Self::MeCentral1),
            "ap_northeast_3" => Ok(Self::ApNortheast3),
            "eu_central_2" => Ok(Self::EuCentral2),
            "mx_central_1" => Ok(Self::MxCentral1),
            "ap_southeast_1" => Ok(Self::ApSoutheast1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GenesysRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UsEast1 => write!(f, "us_east_1"),
            Self::EuWest1 => write!(f, "eu_west_1"),
            Self::ApSoutheast2 => write!(f, "ap_southeast_2"),
            Self::ApNortheast1 => write!(f, "ap_northeast_1"),
            Self::EuCentral1 => write!(f, "eu_central_1"),
            Self::UsWest2 => write!(f, "us_west_2"),
            Self::CaCentral1 => write!(f, "ca_central_1"),
            Self::ApNortheast2 => write!(f, "ap_northeast_2"),
            Self::EuWest2 => write!(f, "eu_west_2"),
            Self::ApSouth1 => write!(f, "ap_south_1"),
            Self::UsEast2 => write!(f, "us_east_2"),
            Self::SaEast1 => write!(f, "sa_east_1"),
            Self::MeCentral1 => write!(f, "me_central_1"),
            Self::ApNortheast3 => write!(f, "ap_northeast_3"),
            Self::EuCentral2 => write!(f, "eu_central_2"),
            Self::MxCentral1 => write!(f, "mx_central_1"),
            Self::ApSoutheast1 => write!(f, "ap_southeast_1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
