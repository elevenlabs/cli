pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlayDtmfResultErrorModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl PlayDtmfResultErrorModel {
    pub fn builder() -> PlayDtmfResultErrorModelBuilder {
        <PlayDtmfResultErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlayDtmfResultErrorModelBuilder {
    status: Option<String>,
    error: Option<String>,
    details: Option<String>,
}

impl PlayDtmfResultErrorModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn details(mut self, value: impl Into<String>) -> Self {
        self.details = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlayDtmfResultErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](PlayDtmfResultErrorModelBuilder::error)
    pub fn build(self) -> Result<PlayDtmfResultErrorModel, BuildError> {
        Ok(PlayDtmfResultErrorModel {
            status: self.status,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
            details: self.details,
        })
    }
}
