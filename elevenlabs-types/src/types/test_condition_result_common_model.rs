pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TestConditionResultCommonModel {
    pub result: EvaluationSuccessResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<TestConditionRationaleCommonModel>,
}

impl TestConditionResultCommonModel {
    pub fn builder() -> TestConditionResultCommonModelBuilder {
        <TestConditionResultCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestConditionResultCommonModelBuilder {
    result: Option<EvaluationSuccessResult>,
    rationale: Option<TestConditionRationaleCommonModel>,
}

impl TestConditionResultCommonModelBuilder {
    pub fn result(mut self, value: EvaluationSuccessResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn rationale(mut self, value: TestConditionRationaleCommonModel) -> Self {
        self.rationale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestConditionResultCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](TestConditionResultCommonModelBuilder::result)
    pub fn build(self) -> Result<TestConditionResultCommonModel, BuildError> {
        Ok(TestConditionResultCommonModel {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            rationale: self.rationale,
        })
    }
}
