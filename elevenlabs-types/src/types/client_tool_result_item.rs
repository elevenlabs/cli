pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A `client_tool_result` whose `type` is required, so that it can share `input` with other items.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClientToolResultItem {
    /// ID of the tool call this result corresponds to.
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub result: String,
    #[serde(default)]
    pub is_error: bool,
    /// Optional category for the failure reason. When set, `is_error` is treated as true. `user_rejected` is special-cased to mark the tool as not having been called.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<ClientToolResultItemErrorType>,
}

impl ClientToolResultItem {
    pub fn builder() -> ClientToolResultItemBuilder {
        <ClientToolResultItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolResultItemBuilder {
    tool_call_id: Option<String>,
    result: Option<String>,
    is_error: Option<bool>,
    error_type: Option<ClientToolResultItemErrorType>,
}

impl ClientToolResultItemBuilder {
    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn result(mut self, value: impl Into<String>) -> Self {
        self.result = Some(value.into());
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn error_type(mut self, value: ClientToolResultItemErrorType) -> Self {
        self.error_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolResultItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](ClientToolResultItemBuilder::tool_call_id)
    /// - [`result`](ClientToolResultItemBuilder::result)
    /// - [`is_error`](ClientToolResultItemBuilder::is_error)
    pub fn build(self) -> Result<ClientToolResultItem, BuildError> {
        Ok(ClientToolResultItem {
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            is_error: self.is_error.ok_or_else(|| BuildError::missing_field("is_error"))?,
            error_type: self.error_type,
        })
    }
}
