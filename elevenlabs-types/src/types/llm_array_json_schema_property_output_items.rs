pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LlmArrayJsonSchemaPropertyOutputItems {
        LlmLiteralJsonSchemaProperty(LlmLiteralJsonSchemaProperty),

        LlmObjectJsonSchemaPropertyOutput(Box<LlmObjectJsonSchemaPropertyOutput>),

        LlmArrayJsonSchemaPropertyOutput(Box<LlmArrayJsonSchemaPropertyOutput>),

        LlmAnyOfJsonSchemaPropertyOutput(LlmAnyOfJsonSchemaPropertyOutput),
}

impl LlmArrayJsonSchemaPropertyOutputItems {
    pub fn is_llm_literal_json_schema_property(&self) -> bool {
        matches!(self, Self::LlmLiteralJsonSchemaProperty(_))
    }

    pub fn is_llm_object_json_schema_property_output(&self) -> bool {
        matches!(self, Self::LlmObjectJsonSchemaPropertyOutput(_))
    }

    pub fn is_llm_array_json_schema_property_output(&self) -> bool {
        matches!(self, Self::LlmArrayJsonSchemaPropertyOutput(_))
    }

    pub fn is_llm_any_of_json_schema_property_output(&self) -> bool {
        matches!(self, Self::LlmAnyOfJsonSchemaPropertyOutput(_))
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

    pub fn as_llm_object_json_schema_property_output(&self) -> Option<&Box<LlmObjectJsonSchemaPropertyOutput>> {
        match self {
                    Self::LlmObjectJsonSchemaPropertyOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_object_json_schema_property_output(self) -> Option<LlmObjectJsonSchemaPropertyOutput> {
        match self {
                    Self::LlmObjectJsonSchemaPropertyOutput(value) => Some(*value),
                    _ => None,
                }
    }

    pub fn as_llm_array_json_schema_property_output(&self) -> Option<&Box<LlmArrayJsonSchemaPropertyOutput>> {
        match self {
                    Self::LlmArrayJsonSchemaPropertyOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_array_json_schema_property_output(self) -> Option<LlmArrayJsonSchemaPropertyOutput> {
        match self {
                    Self::LlmArrayJsonSchemaPropertyOutput(value) => Some(*value),
                    _ => None,
                }
    }

    pub fn as_llm_any_of_json_schema_property_output(&self) -> Option<&LlmAnyOfJsonSchemaPropertyOutput> {
        match self {
                    Self::LlmAnyOfJsonSchemaPropertyOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_llm_any_of_json_schema_property_output(self) -> Option<LlmAnyOfJsonSchemaPropertyOutput> {
        match self {
                    Self::LlmAnyOfJsonSchemaPropertyOutput(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for LlmArrayJsonSchemaPropertyOutputItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LlmLiteralJsonSchemaProperty(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::LlmObjectJsonSchemaPropertyOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::LlmArrayJsonSchemaPropertyOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::LlmAnyOfJsonSchemaPropertyOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
