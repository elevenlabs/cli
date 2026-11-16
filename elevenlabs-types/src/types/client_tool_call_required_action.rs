pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientToolCallRequiredAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

impl ClientToolCallRequiredAction {
    pub fn builder() -> ClientToolCallRequiredActionBuilder {
        <ClientToolCallRequiredActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolCallRequiredActionBuilder {
    r#type: Option<String>,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    parameters: Option<HashMap<String, serde_json::Value>>,
}

impl ClientToolCallRequiredActionBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolCallRequiredAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](ClientToolCallRequiredActionBuilder::tool_call_id)
    /// - [`tool_name`](ClientToolCallRequiredActionBuilder::tool_name)
    /// - [`parameters`](ClientToolCallRequiredActionBuilder::parameters)
    pub fn build(self) -> Result<ClientToolCallRequiredAction, BuildError> {
        Ok(ClientToolCallRequiredAction {
            r#type: self.r#type,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
