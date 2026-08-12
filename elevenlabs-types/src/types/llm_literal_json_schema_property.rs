pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LlmLiteralJsonSchemaProperty {
    pub r#type: LlmLiteralJsonSchemaPropertyType,
    #[serde(default)]
    pub description: String,
    /// List of allowed string values for string type parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
}

impl LlmLiteralJsonSchemaProperty {
    pub fn builder() -> LlmLiteralJsonSchemaPropertyBuilder {
        <LlmLiteralJsonSchemaPropertyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmLiteralJsonSchemaPropertyBuilder {
    r#type: Option<LlmLiteralJsonSchemaPropertyType>,
    description: Option<String>,
    r#enum: Option<Vec<String>>,
}

impl LlmLiteralJsonSchemaPropertyBuilder {
    pub fn r#type(mut self, value: LlmLiteralJsonSchemaPropertyType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn r#enum(mut self, value: Vec<String>) -> Self {
        self.r#enum = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmLiteralJsonSchemaProperty`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LlmLiteralJsonSchemaPropertyBuilder::r#type)
    /// - [`description`](LlmLiteralJsonSchemaPropertyBuilder::description)
    pub fn build(self) -> Result<LlmLiteralJsonSchemaProperty, BuildError> {
        Ok(LlmLiteralJsonSchemaProperty {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            r#enum: self.r#enum,
        })
    }
}
