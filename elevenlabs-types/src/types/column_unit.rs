pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnUnit {
    Ms,
    S,
    Min,
    Duration,
    Credits,
    Usd,
    Eur,
    Inr,
    Pln,
    Ratio,
    Rating,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ColumnUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ms => serializer.serialize_str("ms"),
            Self::S => serializer.serialize_str("s"),
            Self::Min => serializer.serialize_str("min"),
            Self::Duration => serializer.serialize_str("duration"),
            Self::Credits => serializer.serialize_str("credits"),
            Self::Usd => serializer.serialize_str("usd"),
            Self::Eur => serializer.serialize_str("eur"),
            Self::Inr => serializer.serialize_str("inr"),
            Self::Pln => serializer.serialize_str("pln"),
            Self::Ratio => serializer.serialize_str("ratio"),
            Self::Rating => serializer.serialize_str("rating"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ColumnUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ms" => Ok(Self::Ms),
            "s" => Ok(Self::S),
            "min" => Ok(Self::Min),
            "duration" => Ok(Self::Duration),
            "credits" => Ok(Self::Credits),
            "usd" => Ok(Self::Usd),
            "eur" => Ok(Self::Eur),
            "inr" => Ok(Self::Inr),
            "pln" => Ok(Self::Pln),
            "ratio" => Ok(Self::Ratio),
            "rating" => Ok(Self::Rating),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ColumnUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ms => write!(f, "ms"),
            Self::S => write!(f, "s"),
            Self::Min => write!(f, "min"),
            Self::Duration => write!(f, "duration"),
            Self::Credits => write!(f, "credits"),
            Self::Usd => write!(f, "usd"),
            Self::Eur => write!(f, "eur"),
            Self::Inr => write!(f, "inr"),
            Self::Pln => write!(f, "pln"),
            Self::Ratio => write!(f, "ratio"),
            Self::Rating => write!(f, "rating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
