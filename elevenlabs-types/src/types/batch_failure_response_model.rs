pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchFailureResponseModel {
    #[serde(default)]
    pub error_code: i64,
    #[serde(default)]
    pub error_status: String,
    #[serde(default)]
    pub error_message: String,
}

impl BatchFailureResponseModel {
    pub fn builder() -> BatchFailureResponseModelBuilder {
        <BatchFailureResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchFailureResponseModelBuilder {
    error_code: Option<i64>,
    error_status: Option<String>,
    error_message: Option<String>,
}

impl BatchFailureResponseModelBuilder {
    pub fn error_code(mut self, value: i64) -> Self {
        self.error_code = Some(value);
        self
    }

    pub fn error_status(mut self, value: impl Into<String>) -> Self {
        self.error_status = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchFailureResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error_code`](BatchFailureResponseModelBuilder::error_code)
    /// - [`error_status`](BatchFailureResponseModelBuilder::error_status)
    /// - [`error_message`](BatchFailureResponseModelBuilder::error_message)
    pub fn build(self) -> Result<BatchFailureResponseModel, BuildError> {
        Ok(BatchFailureResponseModel {
            error_code: self.error_code.ok_or_else(|| BuildError::missing_field("error_code"))?,
            error_status: self.error_status.ok_or_else(|| BuildError::missing_field("error_status"))?,
            error_message: self.error_message.ok_or_else(|| BuildError::missing_field("error_message"))?,
        })
    }
}
