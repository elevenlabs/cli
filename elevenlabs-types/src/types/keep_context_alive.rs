pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to keep a specific context alive by resetting its inactivity timeout. Empty text is ignored but resets the clock.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KeepContextAlive {
    /// An empty string. This text is ignored by the server but its presence resets the inactivity timeout for the specified context.
    pub text: String,
    /// The identifier of the context to keep alive.
    #[serde(default)]
    pub context_id: String,
}

impl KeepContextAlive {
    pub fn builder() -> KeepContextAliveBuilder {
        <KeepContextAliveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KeepContextAliveBuilder {
    text: Option<String>,
    context_id: Option<String>,
}

impl KeepContextAliveBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KeepContextAlive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](KeepContextAliveBuilder::text)
    /// - [`context_id`](KeepContextAliveBuilder::context_id)
    pub fn build(self) -> Result<KeepContextAlive, BuildError> {
        Ok(KeepContextAlive {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            context_id: self.context_id.ok_or_else(|| BuildError::missing_field("context_id"))?,
        })
    }
}
