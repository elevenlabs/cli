pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct DubbingEndOfStream {
    pub message_type: String,
}

impl DubbingEndOfStream {
    pub fn builder() -> DubbingEndOfStreamBuilder {
        <DubbingEndOfStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingEndOfStreamBuilder {
    message_type: Option<String>,
}

impl DubbingEndOfStreamBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingEndOfStream`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](DubbingEndOfStreamBuilder::message_type)
    pub fn build(self) -> Result<DubbingEndOfStream, BuildError> {
        Ok(DubbingEndOfStream {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
        })
    }
}
