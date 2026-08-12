pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingError {
    /// Stable identifier for the failure, safe to branch on. New codes are added over time, so treat an unrecognized value as 'internal_error'.
    #[serde(default)]
    pub code: String,
    /// Human-readable description of the failure, for display. The wording may change at any time; branch on `code` instead.
    #[serde(default)]
    pub message: String,
    /// Whether resubmitting the same input could succeed. False means the failure describes the input or the account, so an identical retry will fail the same way.
    #[serde(default)]
    pub retryable: bool,
}

impl DubbingError {
    pub fn builder() -> DubbingErrorBuilder {
        <DubbingErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingErrorBuilder {
    code: Option<String>,
    message: Option<String>,
    retryable: Option<bool>,
}

impl DubbingErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](DubbingErrorBuilder::code)
    /// - [`message`](DubbingErrorBuilder::message)
    /// - [`retryable`](DubbingErrorBuilder::retryable)
    pub fn build(self) -> Result<DubbingError, BuildError> {
        Ok(DubbingError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            retryable: self.retryable.ok_or_else(|| BuildError::missing_field("retryable"))?,
        })
    }
}
