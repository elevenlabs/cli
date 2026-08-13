pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The resolution of the output video.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BytedanceSeedance2RequestResolution {
    FourHundredEightyP,
    SevenHundredTwentyP,
    OneThousandEightyP,
    FourK,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BytedanceSeedance2RequestResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FourHundredEightyP => serializer.serialize_str("480p"),
            Self::SevenHundredTwentyP => serializer.serialize_str("720p"),
            Self::OneThousandEightyP => serializer.serialize_str("1080p"),
            Self::FourK => serializer.serialize_str("4k"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BytedanceSeedance2RequestResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "480p" => Ok(Self::FourHundredEightyP),
            "720p" => Ok(Self::SevenHundredTwentyP),
            "1080p" => Ok(Self::OneThousandEightyP),
            "4k" => Ok(Self::FourK),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BytedanceSeedance2RequestResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FourHundredEightyP => write!(f, "480p"),
            Self::SevenHundredTwentyP => write!(f, "720p"),
            Self::OneThousandEightyP => write!(f, "1080p"),
            Self::FourK => write!(f, "4k"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
