pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How to format redacted entities. 'redacted' replaces with {REDACTED}, 'entity_type' replaces with {ENTITY_TYPE}, 'enumerated_entity_type' replaces with {ENTITY_TYPE_N} where N enumerates each occurrence. Only used when entity_redaction is set.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextToDialogueEntityRedactionMode {
    EntityType,
    EnumeratedEntityType,
    Redacted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextToDialogueEntityRedactionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EntityType => serializer.serialize_str("entity_type"),
            Self::EnumeratedEntityType => serializer.serialize_str("enumerated_entity_type"),
            Self::Redacted => serializer.serialize_str("redacted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextToDialogueEntityRedactionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "entity_type" => Ok(Self::EntityType),
            "enumerated_entity_type" => Ok(Self::EnumeratedEntityType),
            "redacted" => Ok(Self::Redacted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextToDialogueEntityRedactionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityType => write!(f, "entity_type"),
            Self::EnumeratedEntityType => write!(f, "enumerated_entity_type"),
            Self::Redacted => write!(f, "redacted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
