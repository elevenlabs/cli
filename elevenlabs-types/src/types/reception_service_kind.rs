pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a service books.
/// 
/// - `classic`: a staff-delivered service (haircut, consultation, massage, ...).
/// - `rental`: an asset reservation (court, room, equipment).
/// - `group`: seats in a multi-participant session (class, workshop, tour).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceptionServiceKind {
    Classic,
    Rental,
    Group,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceptionServiceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Classic => serializer.serialize_str("classic"),
            Self::Rental => serializer.serialize_str("rental"),
            Self::Group => serializer.serialize_str("group"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceptionServiceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "classic" => Ok(Self::Classic),
            "rental" => Ok(Self::Rental),
            "group" => Ok(Self::Group),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceptionServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Classic => write!(f, "classic"),
            Self::Rental => write!(f, "rental"),
            Self::Group => write!(f, "group"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
