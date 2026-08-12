pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClipAnimationExitEffect {
    None,
    Fade,
    Float,
    GentleFloat,
    ZoomIn,
    Drop,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    Pop,
    Bounce,
    Spin,
    SlideBounce,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClipAnimationExitEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Fade => serializer.serialize_str("fade"),
            Self::Float => serializer.serialize_str("float"),
            Self::GentleFloat => serializer.serialize_str("gentle_float"),
            Self::ZoomIn => serializer.serialize_str("zoom_in"),
            Self::Drop => serializer.serialize_str("drop"),
            Self::SlideLeft => serializer.serialize_str("slide_left"),
            Self::SlideRight => serializer.serialize_str("slide_right"),
            Self::SlideUp => serializer.serialize_str("slide_up"),
            Self::SlideDown => serializer.serialize_str("slide_down"),
            Self::Pop => serializer.serialize_str("pop"),
            Self::Bounce => serializer.serialize_str("bounce"),
            Self::Spin => serializer.serialize_str("spin"),
            Self::SlideBounce => serializer.serialize_str("slide_bounce"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClipAnimationExitEffect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "fade" => Ok(Self::Fade),
            "float" => Ok(Self::Float),
            "gentle_float" => Ok(Self::GentleFloat),
            "zoom_in" => Ok(Self::ZoomIn),
            "drop" => Ok(Self::Drop),
            "slide_left" => Ok(Self::SlideLeft),
            "slide_right" => Ok(Self::SlideRight),
            "slide_up" => Ok(Self::SlideUp),
            "slide_down" => Ok(Self::SlideDown),
            "pop" => Ok(Self::Pop),
            "bounce" => Ok(Self::Bounce),
            "spin" => Ok(Self::Spin),
            "slide_bounce" => Ok(Self::SlideBounce),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClipAnimationExitEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Fade => write!(f, "fade"),
            Self::Float => write!(f, "float"),
            Self::GentleFloat => write!(f, "gentle_float"),
            Self::ZoomIn => write!(f, "zoom_in"),
            Self::Drop => write!(f, "drop"),
            Self::SlideLeft => write!(f, "slide_left"),
            Self::SlideRight => write!(f, "slide_right"),
            Self::SlideUp => write!(f, "slide_up"),
            Self::SlideDown => write!(f, "slide_down"),
            Self::Pop => write!(f, "pop"),
            Self::Bounce => write!(f, "bounce"),
            Self::Spin => write!(f, "spin"),
            Self::SlideBounce => write!(f, "slide_bounce"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
