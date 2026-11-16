pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientToolCallClientEventData {
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub event_id: i64,
    /// Whether the server expects a ClientToolResult response.
    #[serde(default)]
    pub expects_response: bool,
}

impl ClientToolCallClientEventData {
    pub fn builder() -> ClientToolCallClientEventDataBuilder {
        <ClientToolCallClientEventDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolCallClientEventDataBuilder {
    tool_name: Option<String>,
    tool_call_id: Option<String>,
    parameters: Option<HashMap<String, serde_json::Value>>,
    event_id: Option<i64>,
    expects_response: Option<bool>,
}

impl ClientToolCallClientEventDataBuilder {
    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn expects_response(mut self, value: bool) -> Self {
        self.expects_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolCallClientEventData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](ClientToolCallClientEventDataBuilder::tool_name)
    /// - [`tool_call_id`](ClientToolCallClientEventDataBuilder::tool_call_id)
    /// - [`parameters`](ClientToolCallClientEventDataBuilder::parameters)
    /// - [`event_id`](ClientToolCallClientEventDataBuilder::event_id)
    /// - [`expects_response`](ClientToolCallClientEventDataBuilder::expects_response)
    pub fn build(self) -> Result<ClientToolCallClientEventData, BuildError> {
        Ok(ClientToolCallClientEventData {
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            expects_response: self.expects_response.ok_or_else(|| BuildError::missing_field("expects_response"))?,
        })
    }
}
