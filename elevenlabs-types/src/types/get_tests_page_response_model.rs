pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTestsPageResponseModel {
    #[serde(default)]
    pub tests: Vec<UnitTestSummaryResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetTestsPageResponseModel {
    pub fn builder() -> GetTestsPageResponseModelBuilder {
        <GetTestsPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTestsPageResponseModelBuilder {
    tests: Option<Vec<UnitTestSummaryResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetTestsPageResponseModelBuilder {
    pub fn tests(mut self, value: Vec<UnitTestSummaryResponseModel>) -> Self {
        self.tests = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTestsPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tests`](GetTestsPageResponseModelBuilder::tests)
    /// - [`has_more`](GetTestsPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetTestsPageResponseModel, BuildError> {
        Ok(GetTestsPageResponseModel {
            tests: self.tests.ok_or_else(|| BuildError::missing_field("tests"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
