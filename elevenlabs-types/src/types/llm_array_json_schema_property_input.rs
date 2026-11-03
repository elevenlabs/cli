pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LlmArrayJsonSchemaPropertyInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub items: Box<LlmArrayJsonSchemaPropertyInputItems>,
}

impl LlmArrayJsonSchemaPropertyInput {
    pub fn builder() -> LlmArrayJsonSchemaPropertyInputBuilder {
        <LlmArrayJsonSchemaPropertyInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmArrayJsonSchemaPropertyInputBuilder {
    r#type: Option<String>,
    description: Option<String>,
    items: Option<Box<LlmArrayJsonSchemaPropertyInputItems>>,
}

impl LlmArrayJsonSchemaPropertyInputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn items(mut self, value: Box<LlmArrayJsonSchemaPropertyInputItems>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmArrayJsonSchemaPropertyInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](LlmArrayJsonSchemaPropertyInputBuilder::items)
    pub fn build(self) -> Result<LlmArrayJsonSchemaPropertyInput, BuildError> {
        Ok(LlmArrayJsonSchemaPropertyInput {
            r#type: self.r#type,
            description: self.description,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
