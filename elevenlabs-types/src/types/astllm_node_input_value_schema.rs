pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AstllmNodeInputValueSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// JSON schema describing the value that the LLM should extract.
    pub value_schema: LlmLiteralJsonSchemaProperty,
}

impl AstllmNodeInputValueSchema {
    pub fn builder() -> AstllmNodeInputValueSchemaBuilder {
        <AstllmNodeInputValueSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstllmNodeInputValueSchemaBuilder {
    r#type: Option<String>,
    value_schema: Option<LlmLiteralJsonSchemaProperty>,
}

impl AstllmNodeInputValueSchemaBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn value_schema(mut self, value: LlmLiteralJsonSchemaProperty) -> Self {
        self.value_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AstllmNodeInputValueSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value_schema`](AstllmNodeInputValueSchemaBuilder::value_schema)
    pub fn build(self) -> Result<AstllmNodeInputValueSchema, BuildError> {
        Ok(AstllmNodeInputValueSchema {
            r#type: self.r#type,
            value_schema: self.value_schema.ok_or_else(|| BuildError::missing_field("value_schema"))?,
        })
    }
}
