pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingInputError {
    pub message_type: String,
    #[serde(default)]
    pub error: String,
}

impl SubtitlingInputError {
    pub fn builder() -> SubtitlingInputErrorBuilder {
        <SubtitlingInputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingInputErrorBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl SubtitlingInputErrorBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingInputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](SubtitlingInputErrorBuilder::message_type)
    /// - [`error`](SubtitlingInputErrorBuilder::error)
    pub fn build(self) -> Result<SubtitlingInputError, BuildError> {
        Ok(SubtitlingInputError {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
