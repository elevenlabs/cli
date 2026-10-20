pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiFunctionTool {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

impl OpenAiFunctionTool {
    pub fn builder() -> OpenAiFunctionToolBuilder {
        <OpenAiFunctionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiFunctionToolBuilder {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<HashMap<String, serde_json::Value>>,
}

impl OpenAiFunctionToolBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiFunctionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OpenAiFunctionToolBuilder::name)
    pub fn build(self) -> Result<OpenAiFunctionTool, BuildError> {
        Ok(OpenAiFunctionTool {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            parameters: self.parameters,
        })
    }
}
