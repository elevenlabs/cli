pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmObjectJsonSchemaPropertyInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<LlmObjectJsonSchemaPropertyInputPropertiesValue>>>,
}

impl LlmObjectJsonSchemaPropertyInput {
    pub fn builder() -> LlmObjectJsonSchemaPropertyInputBuilder {
        <LlmObjectJsonSchemaPropertyInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmObjectJsonSchemaPropertyInputBuilder {
    r#type: Option<String>,
    required: Option<Vec<String>>,
    description: Option<String>,
    properties: Option<HashMap<String, Box<LlmObjectJsonSchemaPropertyInputPropertiesValue>>>,
}

impl LlmObjectJsonSchemaPropertyInputBuilder {
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

    pub fn properties(mut self, value: HashMap<String, Box<LlmObjectJsonSchemaPropertyInputPropertiesValue>>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmObjectJsonSchemaPropertyInput`].
    pub fn build(self) -> Result<LlmObjectJsonSchemaPropertyInput, BuildError> {
        Ok(LlmObjectJsonSchemaPropertyInput {
            r#type: self.r#type,
            required: self.required,
            description: self.description,
            properties: self.properties,
        })
    }
}
