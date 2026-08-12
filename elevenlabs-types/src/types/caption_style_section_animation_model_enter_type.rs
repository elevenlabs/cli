pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptionStyleSectionAnimationModelEnterType {
    None,
    Fade,
    Scale,
    Pop,
    SlideUp,
    SlideDown,
    Slam,
    ScaleDown,
    SlideIn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaptionStyleSectionAnimationModelEnterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Fade => serializer.serialize_str("fade"),
            Self::Scale => serializer.serialize_str("scale"),
            Self::Pop => serializer.serialize_str("pop"),
            Self::SlideUp => serializer.serialize_str("slide_up"),
            Self::SlideDown => serializer.serialize_str("slide_down"),
            Self::Slam => serializer.serialize_str("slam"),
            Self::ScaleDown => serializer.serialize_str("scale_down"),
            Self::SlideIn => serializer.serialize_str("slide_in"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaptionStyleSectionAnimationModelEnterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "fade" => Ok(Self::Fade),
            "scale" => Ok(Self::Scale),
            "pop" => Ok(Self::Pop),
            "slide_up" => Ok(Self::SlideUp),
            "slide_down" => Ok(Self::SlideDown),
            "slam" => Ok(Self::Slam),
            "scale_down" => Ok(Self::ScaleDown),
            "slide_in" => Ok(Self::SlideIn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaptionStyleSectionAnimationModelEnterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Fade => write!(f, "fade"),
            Self::Scale => write!(f, "scale"),
            Self::Pop => write!(f, "pop"),
            Self::SlideUp => write!(f, "slide_up"),
            Self::SlideDown => write!(f, "slide_down"),
            Self::Slam => write!(f, "slam"),
            Self::ScaleDown => write!(f, "scale_down"),
            Self::SlideIn => write!(f, "slide_in"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
