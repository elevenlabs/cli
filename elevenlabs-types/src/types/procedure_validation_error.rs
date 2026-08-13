pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProcedureValidationError {
    /// JSON path to the error, e.g. 'trigger', 'steps[0].instruction'
    #[serde(default)]
    pub path: String,
    /// Human-readable error message
    #[serde(default)]
    pub message: String,
}

impl ProcedureValidationError {
    pub fn builder() -> ProcedureValidationErrorBuilder {
        <ProcedureValidationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcedureValidationErrorBuilder {
    path: Option<String>,
    message: Option<String>,
}

impl ProcedureValidationErrorBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProcedureValidationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](ProcedureValidationErrorBuilder::path)
    /// - [`message`](ProcedureValidationErrorBuilder::message)
    pub fn build(self) -> Result<ProcedureValidationError, BuildError> {
        Ok(ProcedureValidationError {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
