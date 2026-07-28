pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<ValidationError>>,
}

impl HttpValidationError {
    pub fn builder() -> HttpValidationErrorBuilder {
        <HttpValidationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpValidationErrorBuilder {
    detail: Option<Vec<ValidationError>>,
}

impl HttpValidationErrorBuilder {
    pub fn detail(mut self, value: Vec<ValidationError>) -> Self {
        self.detail = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HttpValidationError`].
    pub fn build(self) -> Result<HttpValidationError, BuildError> {
        Ok(HttpValidationError {
            detail: self.detail,
        })
    }
}
