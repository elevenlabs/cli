pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DashboardCriteriaChartModel {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub criteria_id: String,
}

impl DashboardCriteriaChartModel {
    pub fn builder() -> DashboardCriteriaChartModelBuilder {
        <DashboardCriteriaChartModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DashboardCriteriaChartModelBuilder {
    name: Option<String>,
    criteria_id: Option<String>,
}

impl DashboardCriteriaChartModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn criteria_id(mut self, value: impl Into<String>) -> Self {
        self.criteria_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DashboardCriteriaChartModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DashboardCriteriaChartModelBuilder::name)
    /// - [`criteria_id`](DashboardCriteriaChartModelBuilder::criteria_id)
    pub fn build(self) -> Result<DashboardCriteriaChartModel, BuildError> {
        Ok(DashboardCriteriaChartModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            criteria_id: self.criteria_id.ok_or_else(|| BuildError::missing_field("criteria_id"))?,
        })
    }
}
