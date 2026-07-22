pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunSubagentToolResultSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub agent_response: String,
}

impl RunSubagentToolResultSuccessModel {
    pub fn builder() -> RunSubagentToolResultSuccessModelBuilder {
        <RunSubagentToolResultSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunSubagentToolResultSuccessModelBuilder {
    status: Option<String>,
    query: Option<String>,
    agent_response: Option<String>,
}

impl RunSubagentToolResultSuccessModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn agent_response(mut self, value: impl Into<String>) -> Self {
        self.agent_response = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunSubagentToolResultSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](RunSubagentToolResultSuccessModelBuilder::query)
    /// - [`agent_response`](RunSubagentToolResultSuccessModelBuilder::agent_response)
    pub fn build(self) -> Result<RunSubagentToolResultSuccessModel, BuildError> {
        Ok(RunSubagentToolResultSuccessModel {
            status: self.status,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            agent_response: self.agent_response.ok_or_else(|| BuildError::missing_field("agent_response"))?,
        })
    }
}
