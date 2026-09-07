pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryParamsJsonSchemaInput {
    #[serde(default)]
    pub properties: HashMap<String, LiteralJsonSchemaProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl QueryParamsJsonSchemaInput {
    pub fn builder() -> QueryParamsJsonSchemaInputBuilder {
        <QueryParamsJsonSchemaInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryParamsJsonSchemaInputBuilder {
    properties: Option<HashMap<String, LiteralJsonSchemaProperty>>,
    required: Option<Vec<String>>,
}

impl QueryParamsJsonSchemaInputBuilder {
    pub fn properties(mut self, value: HashMap<String, LiteralJsonSchemaProperty>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryParamsJsonSchemaInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`properties`](QueryParamsJsonSchemaInputBuilder::properties)
    pub fn build(self) -> Result<QueryParamsJsonSchemaInput, BuildError> {
        Ok(QueryParamsJsonSchemaInput {
            properties: self.properties.ok_or_else(|| BuildError::missing_field("properties"))?,
            required: self.required,
        })
    }
}
