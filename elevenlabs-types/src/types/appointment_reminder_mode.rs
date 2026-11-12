pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppointmentReminderMode {
    Reminder,
    Confirmation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppointmentReminderMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reminder => serializer.serialize_str("reminder"),
            Self::Confirmation => serializer.serialize_str("confirmation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppointmentReminderMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reminder" => Ok(Self::Reminder),
            "confirmation" => Ok(Self::Confirmation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppointmentReminderMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reminder => write!(f, "reminder"),
            Self::Confirmation => write!(f, "confirmation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
