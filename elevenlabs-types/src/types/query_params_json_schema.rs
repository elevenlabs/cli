pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryParamsJsonSchema {
    #[serde(default)]
    pub properties: HashMap<String, LiteralJsonSchemaProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl QueryParamsJsonSchema {
    pub fn builder() -> QueryParamsJsonSchemaBuilder {
        <QueryParamsJsonSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryParamsJsonSchemaBuilder {
    properties: Option<HashMap<String, LiteralJsonSchemaProperty>>,
    required: Option<Vec<String>>,
}

impl QueryParamsJsonSchemaBuilder {
    pub fn properties(mut self, value: HashMap<String, LiteralJsonSchemaProperty>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryParamsJsonSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`properties`](QueryParamsJsonSchemaBuilder::properties)
    pub fn build(self) -> Result<QueryParamsJsonSchema, BuildError> {
        Ok(QueryParamsJsonSchema {
            properties: self.properties.ok_or_else(|| BuildError::missing_field("properties"))?,
            required: self.required,
        })
    }
}
