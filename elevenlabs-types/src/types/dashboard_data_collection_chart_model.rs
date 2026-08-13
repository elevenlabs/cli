pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DashboardDataCollectionChartModel {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub data_collection_id: String,
}

impl DashboardDataCollectionChartModel {
    pub fn builder() -> DashboardDataCollectionChartModelBuilder {
        <DashboardDataCollectionChartModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DashboardDataCollectionChartModelBuilder {
    name: Option<String>,
    data_collection_id: Option<String>,
}

impl DashboardDataCollectionChartModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn data_collection_id(mut self, value: impl Into<String>) -> Self {
        self.data_collection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DashboardDataCollectionChartModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DashboardDataCollectionChartModelBuilder::name)
    /// - [`data_collection_id`](DashboardDataCollectionChartModelBuilder::data_collection_id)
    pub fn build(self) -> Result<DashboardDataCollectionChartModel, BuildError> {
        Ok(DashboardDataCollectionChartModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            data_collection_id: self.data_collection_id.ok_or_else(|| BuildError::missing_field("data_collection_id"))?,
        })
    }
}
