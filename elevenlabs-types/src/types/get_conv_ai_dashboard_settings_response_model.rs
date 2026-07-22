pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetConvAiDashboardSettingsResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charts: Option<Vec<GetConvAiDashboardSettingsResponseModelChartsItem>>,
}

impl GetConvAiDashboardSettingsResponseModel {
    pub fn builder() -> GetConvAiDashboardSettingsResponseModelBuilder {
        <GetConvAiDashboardSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConvAiDashboardSettingsResponseModelBuilder {
    charts: Option<Vec<GetConvAiDashboardSettingsResponseModelChartsItem>>,
}

impl GetConvAiDashboardSettingsResponseModelBuilder {
    pub fn charts(mut self, value: Vec<GetConvAiDashboardSettingsResponseModelChartsItem>) -> Self {
        self.charts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetConvAiDashboardSettingsResponseModel`].
    pub fn build(self) -> Result<GetConvAiDashboardSettingsResponseModel, BuildError> {
        Ok(GetConvAiDashboardSettingsResponseModel {
            charts: self.charts,
        })
    }
}
