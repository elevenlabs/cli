pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ObjectJsonSchemaPropertyInputPropertiesValue {
        LiteralJsonSchemaProperty(LiteralJsonSchemaProperty),

        ObjectJsonSchemaPropertyInput(Box<ObjectJsonSchemaPropertyInput>),

        ArrayJsonSchemaPropertyInput(Box<ArrayJsonSchemaPropertyInput>),
}

impl ObjectJsonSchemaPropertyInputPropertiesValue {
    pub fn is_literal_json_schema_property(&self) -> bool {
        matches!(self, Self::LiteralJsonSchemaProperty(_))
    }

    pub fn is_object_json_schema_property_input(&self) -> bool {
        matches!(self, Self::ObjectJsonSchemaPropertyInput(_))
    }

    pub fn is_array_json_schema_property_input(&self) -> bool {
        matches!(self, Self::ArrayJsonSchemaPropertyInput(_))
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

    pub fn as_object_json_schema_property_input(&self) -> Option<&Box<ObjectJsonSchemaPropertyInput>> {
        match self {
                    Self::ObjectJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_object_json_schema_property_input(self) -> Option<ObjectJsonSchemaPropertyInput> {
        match self {
                    Self::ObjectJsonSchemaPropertyInput(value) => Some(*value),
                    _ => None,
                }
    }

    pub fn as_array_json_schema_property_input(&self) -> Option<&Box<ArrayJsonSchemaPropertyInput>> {
        match self {
                    Self::ArrayJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_array_json_schema_property_input(self) -> Option<ArrayJsonSchemaPropertyInput> {
        match self {
                    Self::ArrayJsonSchemaPropertyInput(value) => Some(*value),
                    _ => None,
                }
    }
}

impl fmt::Display for ObjectJsonSchemaPropertyInputPropertiesValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiteralJsonSchemaProperty(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ObjectJsonSchemaPropertyInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ArrayJsonSchemaPropertyInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
