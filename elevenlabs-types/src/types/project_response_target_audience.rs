pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectResponseTargetAudience {
    Children,
    YoungAdult,
    Adult,
    AllAges,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectResponseTargetAudience {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Children => serializer.serialize_str("children"),
            Self::YoungAdult => serializer.serialize_str("young adult"),
            Self::Adult => serializer.serialize_str("adult"),
            Self::AllAges => serializer.serialize_str("all ages"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectResponseTargetAudience {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "children" => Ok(Self::Children),
            "young adult" => Ok(Self::YoungAdult),
            "adult" => Ok(Self::Adult),
            "all ages" => Ok(Self::AllAges),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectResponseTargetAudience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Children => write!(f, "children"),
            Self::YoungAdult => write!(f, "young adult"),
            Self::Adult => write!(f, "adult"),
            Self::AllAges => write!(f, "all ages"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
