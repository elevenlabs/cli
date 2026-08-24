pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchConvAiDashboardSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charts: Option<Vec<PatchConvAiDashboardSettingsRequestChartsItem>>,
}

impl PatchConvAiDashboardSettingsRequest {
    pub fn builder() -> PatchConvAiDashboardSettingsRequestBuilder {
        <PatchConvAiDashboardSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchConvAiDashboardSettingsRequestBuilder {
    charts: Option<Vec<PatchConvAiDashboardSettingsRequestChartsItem>>,
}

impl PatchConvAiDashboardSettingsRequestBuilder {
    pub fn charts(mut self, value: Vec<PatchConvAiDashboardSettingsRequestChartsItem>) -> Self {
        self.charts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchConvAiDashboardSettingsRequest`].
    pub fn build(self) -> Result<PatchConvAiDashboardSettingsRequest, BuildError> {
        Ok(PatchConvAiDashboardSettingsRequest {
            charts: self.charts,
        })
    }
}

