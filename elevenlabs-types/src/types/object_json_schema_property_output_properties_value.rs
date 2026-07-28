pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ObjectJsonSchemaPropertyOutputPropertiesValue {
        LiteralJsonSchemaProperty(LiteralJsonSchemaProperty),

        ObjectJsonSchemaPropertyOutput(Box<ObjectJsonSchemaPropertyOutput>),

        ArrayJsonSchemaPropertyOutput(Box<ArrayJsonSchemaPropertyOutput>),
}

impl ObjectJsonSchemaPropertyOutputPropertiesValue {
    pub fn is_literal_json_schema_property(&self) -> bool {
        matches!(self, Self::LiteralJsonSchemaProperty(_))
    }

    pub fn is_object_json_schema_property_output(&self) -> bool {
        matches!(self, Self::ObjectJsonSchemaPropertyOutput(_))
    }

    pub fn is_array_json_schema_property_output(&self) -> bool {
        matches!(self, Self::ArrayJsonSchemaPropertyOutput(_))
    }


    pub fn as_literal_json_schema_property(&self) -> Option<&LiteralJsonSchemaProperty> {
        match self {
                    Self::LiteralJsonSchemaProperty(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_literal_json_schema_property(self) -> Option<LiteralJsonSchemaProperty> {
        match self {
                    Self::LiteralJsonSchemaProperty(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_object_json_schema_property_output(&self) -> Option<&Box<ObjectJsonSchemaPropertyOutput>> {
        match self {
                    Self::ObjectJsonSchemaPropertyOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_object_json_schema_property_output(self) -> Option<ObjectJsonSchemaPropertyOutput> {
        match self {
                    Self::ObjectJsonSchemaPropertyOutput(value) => Some(*value),
                    _ => None,
                }
    }

    pub fn as_array_json_schema_property_output(&self) -> Option<&Box<ArrayJsonSchemaPropertyOutput>> {
        match self {
                    Self::ArrayJsonSchemaPropertyOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_array_json_schema_property_output(self) -> Option<ArrayJsonSchemaPropertyOutput> {
        match self {
                    Self::ArrayJsonSchemaPropertyOutput(value) => Some(*value),
                    _ => None,
                }
    }
}

impl fmt::Display for ObjectJsonSchemaPropertyOutputPropertiesValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiteralJsonSchemaProperty(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ObjectJsonSchemaPropertyOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ArrayJsonSchemaPropertyOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
