pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestsFeatureUsageCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests_ran_after_last_modification: Option<bool>,
    #[serde(rename = "tests_ran_in_last_7_days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests_ran_in_last7days: Option<bool>,
}

impl TestsFeatureUsageCommonModel {
    pub fn builder() -> TestsFeatureUsageCommonModelBuilder {
        <TestsFeatureUsageCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestsFeatureUsageCommonModelBuilder {
    enabled: Option<bool>,
    tests_ran_after_last_modification: Option<bool>,
    tests_ran_in_last7days: Option<bool>,
}

impl TestsFeatureUsageCommonModelBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn tests_ran_after_last_modification(mut self, value: bool) -> Self {
        self.tests_ran_after_last_modification = Some(value);
        self
    }

    pub fn tests_ran_in_last7days(mut self, value: bool) -> Self {
        self.tests_ran_in_last7days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestsFeatureUsageCommonModel`].
    pub fn build(self) -> Result<TestsFeatureUsageCommonModel, BuildError> {
        Ok(TestsFeatureUsageCommonModel {
            enabled: self.enabled,
            tests_ran_after_last_modification: self.tests_ran_after_last_modification,
            tests_ran_in_last7days: self.tests_ran_in_last7days,
        })
    }
}
