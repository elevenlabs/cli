pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A query param holding several literals, sent as a repeated key (?a=x&a=y).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryArrayJsonSchemaPropertyOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When set, the entire parameter is populated from this dynamic variable at runtime. Mutually exclusive with description (LLM-provided value), constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable: Option<String>,
    /// When set, the entire array uses this constant value at runtime. Mutually exclusive with description (LLM-provided array), dynamic_variable, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_value: Option<Vec<serde_json::Value>>,
    /// If true, this parameter will be completely omitted from the request. Only valid for optional parameters. Mutually exclusive with description, dynamic_variable, and constant_value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_omitted: Option<bool>,
    pub r#type: String,
    /// Schema for each value; must be a string, number or boolean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<QueryArrayJsonSchemaPropertyOutputItems>,
}

impl QueryArrayJsonSchemaPropertyOutput {
    pub fn builder() -> QueryArrayJsonSchemaPropertyOutputBuilder {
        <QueryArrayJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryArrayJsonSchemaPropertyOutputBuilder {
    description: Option<String>,
    dynamic_variable: Option<String>,
    constant_value: Option<Vec<serde_json::Value>>,
    is_omitted: Option<bool>,
    r#type: Option<String>,
    items: Option<QueryArrayJsonSchemaPropertyOutputItems>,
}

impl QueryArrayJsonSchemaPropertyOutputBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    pub fn constant_value(mut self, value: Vec<serde_json::Value>) -> Self {
        self.constant_value = Some(value);
        self
    }

    pub fn is_omitted(mut self, value: bool) -> Self {
        self.is_omitted = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn items(mut self, value: QueryArrayJsonSchemaPropertyOutputItems) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryArrayJsonSchemaPropertyOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](QueryArrayJsonSchemaPropertyOutputBuilder::r#type)
    pub fn build(self) -> Result<QueryArrayJsonSchemaPropertyOutput, BuildError> {
        Ok(QueryArrayJsonSchemaPropertyOutput {
            description: self.description,
            dynamic_variable: self.dynamic_variable,
            constant_value: self.constant_value,
            is_omitted: self.is_omitted,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            items: self.items,
        })
    }
}
