pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Structured rationale for test condition results containing individual failure/success reasons.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestConditionRationaleCommonModel {
    /// List of individual parameter evaluation messages or reasons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    /// High-level summary of the evaluation result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl TestConditionRationaleCommonModel {
    pub fn builder() -> TestConditionRationaleCommonModelBuilder {
        <TestConditionRationaleCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestConditionRationaleCommonModelBuilder {
    messages: Option<Vec<String>>,
    summary: Option<String>,
}

impl TestConditionRationaleCommonModelBuilder {
    pub fn messages(mut self, value: Vec<String>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TestConditionRationaleCommonModel`].
    pub fn build(self) -> Result<TestConditionRationaleCommonModel, BuildError> {
        Ok(TestConditionRationaleCommonModel {
            messages: self.messages,
            summary: self.summary,
        })
    }
}
