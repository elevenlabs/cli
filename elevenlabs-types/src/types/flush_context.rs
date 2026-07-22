pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to flush the audio buffer for a specific context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlushContext {
    /// The context_id to flush.
    #[serde(default)]
    pub context_id: String,
    /// The text to append to the buffer to be flushed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If true, flushes the audio buffer for the specified context. If false, the context will remain open and the text will be appended to the buffer to be generated.
    #[serde(default)]
    pub flush: bool,
}

impl FlushContext {
    pub fn builder() -> FlushContextBuilder {
        <FlushContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlushContextBuilder {
    context_id: Option<String>,
    text: Option<String>,
    flush: Option<bool>,
}

impl FlushContextBuilder {
    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FlushContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_id`](FlushContextBuilder::context_id)
    /// - [`flush`](FlushContextBuilder::flush)
    pub fn build(self) -> Result<FlushContext, BuildError> {
        Ok(FlushContext {
            context_id: self.context_id.ok_or_else(|| BuildError::missing_field("context_id"))?,
            text: self.text,
            flush: self.flush.ok_or_else(|| BuildError::missing_field("flush"))?,
        })
    }
}
