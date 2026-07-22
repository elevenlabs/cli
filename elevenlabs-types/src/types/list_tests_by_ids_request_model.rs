pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTestsByIdsRequestModel {
    /// List of test IDs to fetch. No duplicates allowed.
    #[serde(default)]
    pub test_ids: Vec<String>,
}

impl ListTestsByIdsRequestModel {
    pub fn builder() -> ListTestsByIdsRequestModelBuilder {
        <ListTestsByIdsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTestsByIdsRequestModelBuilder {
    test_ids: Option<Vec<String>>,
}

impl ListTestsByIdsRequestModelBuilder {
    pub fn test_ids(mut self, value: Vec<String>) -> Self {
        self.test_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTestsByIdsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_ids`](ListTestsByIdsRequestModelBuilder::test_ids)
    pub fn build(self) -> Result<ListTestsByIdsRequestModel, BuildError> {
        Ok(ListTestsByIdsRequestModel {
            test_ids: self.test_ids.ok_or_else(|| BuildError::missing_field("test_ids"))?,
        })
    }
}

