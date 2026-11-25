pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryParamsJsonSchemaInputPropertiesValue {
        LiteralJsonSchemaProperty(LiteralJsonSchemaProperty),

        QueryArrayJsonSchemaPropertyInput(QueryArrayJsonSchemaPropertyInput),
}

impl QueryParamsJsonSchemaInputPropertiesValue {
    pub fn is_literal_json_schema_property(&self) -> bool {
        matches!(self, Self::LiteralJsonSchemaProperty(_))
    }

    pub fn is_query_array_json_schema_property_input(&self) -> bool {
        matches!(self, Self::QueryArrayJsonSchemaPropertyInput(_))
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

    pub fn as_query_array_json_schema_property_input(&self) -> Option<&QueryArrayJsonSchemaPropertyInput> {
        match self {
                    Self::QueryArrayJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_query_array_json_schema_property_input(self) -> Option<QueryArrayJsonSchemaPropertyInput> {
        match self {
                    Self::QueryArrayJsonSchemaPropertyInput(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for QueryParamsJsonSchemaInputPropertiesValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiteralJsonSchemaProperty(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::QueryArrayJsonSchemaPropertyInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
