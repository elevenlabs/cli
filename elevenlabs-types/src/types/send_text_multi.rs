pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to send text for synthesis to an existing context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTextMulti {
    /// Text to synthesize. Should end with a single space.
    #[serde(default)]
    pub text: String,
    /// The target context_id for this text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// If true, flushes the audio buffer for the specified context. If false, the text will be appended to the buffer to be generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush: Option<bool>,
}

impl SendTextMulti {
    pub fn builder() -> SendTextMultiBuilder {
        <SendTextMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTextMultiBuilder {
    text: Option<String>,
    context_id: Option<String>,
    flush: Option<bool>,
}

impl SendTextMultiBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTextMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SendTextMultiBuilder::text)
    pub fn build(self) -> Result<SendTextMulti, BuildError> {
        Ok(SendTextMulti {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            context_id: self.context_id,
            flush: self.flush,
        })
    }
}
