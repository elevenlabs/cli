pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextToDialogueWebsocketError {
    /// Human-readable error description.
    #[serde(default)]
    pub message: String,
    /// Machine-readable error identifier (for example `authentication_required`).
    #[serde(default)]
    pub error: String,
    /// WebSocket close code that will follow this payload.
    #[serde(default)]
    pub code: i64,
    /// Field name related to the error, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl TextToDialogueWebsocketError {
    pub fn builder() -> TextToDialogueWebsocketErrorBuilder {
        <TextToDialogueWebsocketErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketErrorBuilder {
    message: Option<String>,
    error: Option<String>,
    code: Option<i64>,
    param: Option<String>,
}

impl TextToDialogueWebsocketErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn param(mut self, value: impl Into<String>) -> Self {
        self.param = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](TextToDialogueWebsocketErrorBuilder::message)
    /// - [`error`](TextToDialogueWebsocketErrorBuilder::error)
    /// - [`code`](TextToDialogueWebsocketErrorBuilder::code)
    pub fn build(self) -> Result<TextToDialogueWebsocketError, BuildError> {
        Ok(TextToDialogueWebsocketError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            param: self.param,
        })
    }
}
