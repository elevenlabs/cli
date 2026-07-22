pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BadRequestErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BadRequestErrorBody {
    pub fn builder() -> BadRequestErrorBodyBuilder {
        <BadRequestErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadRequestErrorBodyBuilder {
    error: Option<String>,
    message: Option<String>,
}

impl BadRequestErrorBodyBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BadRequestErrorBody`].
    pub fn build(self) -> Result<BadRequestErrorBody, BuildError> {
        Ok(BadRequestErrorBody {
            error: self.error,
            message: self.message,
        })
    }
}
