pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to close a specific TTS context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CloseContext {
    /// The context_id to close.
    #[serde(default)]
    pub context_id: String,
    /// Must set the close_context to true, to close the specified context. If false, the context will remain open and the text will be ignored. If set to true, the context will close. If it has already been set to flush it will continue flushing. The same context id can be used again but will not be linked to the previous context with the same name.
    #[serde(default)]
    pub close_context: bool,
}

impl CloseContext {
    pub fn builder() -> CloseContextBuilder {
        <CloseContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloseContextBuilder {
    context_id: Option<String>,
    close_context: Option<bool>,
}

impl CloseContextBuilder {
    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn close_context(mut self, value: bool) -> Self {
        self.close_context = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CloseContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_id`](CloseContextBuilder::context_id)
    /// - [`close_context`](CloseContextBuilder::close_context)
    pub fn build(self) -> Result<CloseContext, BuildError> {
        Ok(CloseContext {
            context_id: self.context_id.ok_or_else(|| BuildError::missing_field("context_id"))?,
            close_context: self.close_context.ok_or_else(|| BuildError::missing_field("close_context"))?,
        })
    }
}
