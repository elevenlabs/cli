pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DashboardCallSuccessChartModel {
    #[serde(default)]
    pub name: String,
}

impl DashboardCallSuccessChartModel {
    pub fn builder() -> DashboardCallSuccessChartModelBuilder {
        <DashboardCallSuccessChartModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DashboardCallSuccessChartModelBuilder {
    name: Option<String>,
}

impl DashboardCallSuccessChartModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DashboardCallSuccessChartModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DashboardCallSuccessChartModelBuilder::name)
    pub fn build(self) -> Result<DashboardCallSuccessChartModel, BuildError> {
        Ok(DashboardCallSuccessChartModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
