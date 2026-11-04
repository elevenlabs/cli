pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LlmArrayJsonSchemaPropertyOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub items: Box<LlmArrayJsonSchemaPropertyOutputItems>,
}

impl LlmArrayJsonSchemaPropertyOutput {
    pub fn builder() -> LlmArrayJsonSchemaPropertyOutputBuilder {
        <LlmArrayJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmArrayJsonSchemaPropertyOutputBuilder {
    r#type: Option<String>,
    description: Option<String>,
    items: Option<Box<LlmArrayJsonSchemaPropertyOutputItems>>,
}

impl LlmArrayJsonSchemaPropertyOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn items(mut self, value: Box<LlmArrayJsonSchemaPropertyOutputItems>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmArrayJsonSchemaPropertyOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](LlmArrayJsonSchemaPropertyOutputBuilder::items)
    pub fn build(self) -> Result<LlmArrayJsonSchemaPropertyOutput, BuildError> {
        Ok(LlmArrayJsonSchemaPropertyOutput {
            r#type: self.r#type,
            description: self.description,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
