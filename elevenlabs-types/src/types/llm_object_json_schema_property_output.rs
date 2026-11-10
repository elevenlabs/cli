pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmObjectJsonSchemaPropertyOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<LlmObjectJsonSchemaPropertyOutputPropertiesValue>>>,
}

impl LlmObjectJsonSchemaPropertyOutput {
    pub fn builder() -> LlmObjectJsonSchemaPropertyOutputBuilder {
        <LlmObjectJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmObjectJsonSchemaPropertyOutputBuilder {
    r#type: Option<String>,
    required: Option<Vec<String>>,
    description: Option<String>,
    properties: Option<HashMap<String, Box<LlmObjectJsonSchemaPropertyOutputPropertiesValue>>>,
}

impl LlmObjectJsonSchemaPropertyOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn properties(mut self, value: HashMap<String, Box<LlmObjectJsonSchemaPropertyOutputPropertiesValue>>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmObjectJsonSchemaPropertyOutput`].
    pub fn build(self) -> Result<LlmObjectJsonSchemaPropertyOutput, BuildError> {
        Ok(LlmObjectJsonSchemaPropertyOutput {
            r#type: self.r#type,
            required: self.required,
            description: self.description,
            properties: self.properties,
        })
    }
}
