pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestToolResultModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_node: Option<String>,
}

impl TestToolResultModel {
    pub fn builder() -> TestToolResultModelBuilder {
        <TestToolResultModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestToolResultModelBuilder {
    status: Option<String>,
    reason: Option<String>,
    to_agent: Option<String>,
    to_node: Option<String>,
}

impl TestToolResultModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn to_agent(mut self, value: impl Into<String>) -> Self {
        self.to_agent = Some(value.into());
        self
    }

    pub fn to_node(mut self, value: impl Into<String>) -> Self {
        self.to_node = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TestToolResultModel`].
    pub fn build(self) -> Result<TestToolResultModel, BuildError> {
        Ok(TestToolResultModel {
            status: self.status,
            reason: self.reason,
            to_agent: self.to_agent,
            to_node: self.to_node,
        })
    }
}
