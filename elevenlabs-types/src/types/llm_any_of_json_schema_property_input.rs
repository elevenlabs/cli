pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmAnyOfJsonSchemaPropertyInput {
    #[serde(default)]
    pub any_of: Vec<Box<LlmObjectJsonSchemaPropertyInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl LlmAnyOfJsonSchemaPropertyInput {
    pub fn builder() -> LlmAnyOfJsonSchemaPropertyInputBuilder {
        <LlmAnyOfJsonSchemaPropertyInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmAnyOfJsonSchemaPropertyInputBuilder {
    any_of: Option<Vec<Box<LlmObjectJsonSchemaPropertyInput>>>,
    description: Option<String>,
}

impl LlmAnyOfJsonSchemaPropertyInputBuilder {
    pub fn any_of(mut self, value: Vec<Box<LlmObjectJsonSchemaPropertyInput>>) -> Self {
        self.any_of = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LlmAnyOfJsonSchemaPropertyInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`any_of`](LlmAnyOfJsonSchemaPropertyInputBuilder::any_of)
    pub fn build(self) -> Result<LlmAnyOfJsonSchemaPropertyInput, BuildError> {
        Ok(LlmAnyOfJsonSchemaPropertyInput {
            any_of: self.any_of.ok_or_else(|| BuildError::missing_field("any_of"))?,
            description: self.description,
        })
    }
}
