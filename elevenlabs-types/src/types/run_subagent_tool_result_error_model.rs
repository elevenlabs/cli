pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunSubagentToolResultErrorModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub error: String,
}

impl RunSubagentToolResultErrorModel {
    pub fn builder() -> RunSubagentToolResultErrorModelBuilder {
        <RunSubagentToolResultErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunSubagentToolResultErrorModelBuilder {
    status: Option<String>,
    error: Option<String>,
}

impl RunSubagentToolResultErrorModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunSubagentToolResultErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](RunSubagentToolResultErrorModelBuilder::error)
    pub fn build(self) -> Result<RunSubagentToolResultErrorModel, BuildError> {
        Ok(RunSubagentToolResultErrorModel {
            status: self.status,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
