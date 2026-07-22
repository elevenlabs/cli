pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndCallToolResultModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl EndCallToolResultModel {
    pub fn builder() -> EndCallToolResultModelBuilder {
        <EndCallToolResultModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndCallToolResultModelBuilder {
    status: Option<String>,
    reason: Option<String>,
    message: Option<String>,
}

impl EndCallToolResultModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EndCallToolResultModel`].
    pub fn build(self) -> Result<EndCallToolResultModel, BuildError> {
        Ok(EndCallToolResultModel {
            status: self.status,
            reason: self.reason,
            message: self.message,
        })
    }
}
