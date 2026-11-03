pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LlmObjectJsonSchemaPropertyInputPropertiesValue {
        LlmLiteralJsonSchemaProperty(LlmLiteralJsonSchemaProperty),

        LlmObjectJsonSchemaPropertyInput(Box<LlmObjectJsonSchemaPropertyInput>),

        LlmArrayJsonSchemaPropertyInput(Box<LlmArrayJsonSchemaPropertyInput>),

        LlmAnyOfJsonSchemaPropertyInput(Box<LlmAnyOfJsonSchemaPropertyInput>),
}

impl LlmObjectJsonSchemaPropertyInputPropertiesValue {
    pub fn is_llm_literal_json_schema_property(&self) -> bool {
        matches!(self, Self::LlmLiteralJsonSchemaProperty(_))
    }

    pub fn is_llm_object_json_schema_property_input(&self) -> bool {
        matches!(self, Self::LlmObjectJsonSchemaPropertyInput(_))
    }

    pub fn is_llm_array_json_schema_property_input(&self) -> bool {
        matches!(self, Self::LlmArrayJsonSchemaPropertyInput(_))
    }

    pub fn is_llm_any_of_json_schema_property_input(&self) -> bool {
        matches!(self, Self::LlmAnyOfJsonSchemaPropertyInput(_))
    }


    pub fn as_llm_literal_json_schema_property(&self) -> Option<&LlmLiteralJsonSchemaProperty> {
        match self {
                    Self::LlmLiteralJsonSchemaProperty(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_literal_json_schema_property(self) -> Option<LlmLiteralJsonSchemaProperty> {
        match self {
                    Self::LlmLiteralJsonSchemaProperty(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_llm_object_json_schema_property_input(&self) -> Option<&Box<LlmObjectJsonSchemaPropertyInput>> {
        match self {
                    Self::LlmObjectJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_object_json_schema_property_input(self) -> Option<LlmObjectJsonSchemaPropertyInput> {
        match self {
                    Self::LlmObjectJsonSchemaPropertyInput(value) => Some(*value),
                    _ => None,
                }
    }

    pub fn as_llm_array_json_schema_property_input(&self) -> Option<&Box<LlmArrayJsonSchemaPropertyInput>> {
        match self {
                    Self::LlmArrayJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_array_json_schema_property_input(self) -> Option<LlmArrayJsonSchemaPropertyInput> {
        match self {
                    Self::LlmArrayJsonSchemaPropertyInput(value) => Some(*value),
                    _ => None,
                }
    }

    pub fn as_llm_any_of_json_schema_property_input(&self) -> Option<&Box<LlmAnyOfJsonSchemaPropertyInput>> {
        match self {
                    Self::LlmAnyOfJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_any_of_json_schema_property_input(self) -> Option<LlmAnyOfJsonSchemaPropertyInput> {
        match self {
                    Self::LlmAnyOfJsonSchemaPropertyInput(value) => Some(*value),
                    _ => None,
                }
    }
}

impl fmt::Display for LlmObjectJsonSchemaPropertyInputPropertiesValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LlmLiteralJsonSchemaProperty(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::LlmObjectJsonSchemaPropertyInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::LlmArrayJsonSchemaPropertyInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::LlmAnyOfJsonSchemaPropertyInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
