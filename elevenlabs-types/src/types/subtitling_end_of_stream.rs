pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SubtitlingEndOfStream {
    pub message_type: String,
}

impl SubtitlingEndOfStream {
    pub fn builder() -> SubtitlingEndOfStreamBuilder {
        <SubtitlingEndOfStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingEndOfStreamBuilder {
    message_type: Option<String>,
}

impl SubtitlingEndOfStreamBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingEndOfStream`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](SubtitlingEndOfStreamBuilder::message_type)
    pub fn build(self) -> Result<SubtitlingEndOfStream, BuildError> {
        Ok(SubtitlingEndOfStream {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
        })
    }
}
