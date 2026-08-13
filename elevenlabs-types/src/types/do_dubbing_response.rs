pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DoDubbingResponse {
    /// The ID of the dubbing project.
    #[serde(default)]
    pub dubbing_id: String,
    /// The expected duration of the dubbing project in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub expected_duration_sec: f64,
}

impl DoDubbingResponse {
    pub fn builder() -> DoDubbingResponseBuilder {
        <DoDubbingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DoDubbingResponseBuilder {
    dubbing_id: Option<String>,
    expected_duration_sec: Option<f64>,
}

impl DoDubbingResponseBuilder {
    pub fn dubbing_id(mut self, value: impl Into<String>) -> Self {
        self.dubbing_id = Some(value.into());
        self
    }

    pub fn expected_duration_sec(mut self, value: f64) -> Self {
        self.expected_duration_sec = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DoDubbingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dubbing_id`](DoDubbingResponseBuilder::dubbing_id)
    /// - [`expected_duration_sec`](DoDubbingResponseBuilder::expected_duration_sec)
    pub fn build(self) -> Result<DoDubbingResponse, BuildError> {
        Ok(DoDubbingResponse {
            dubbing_id: self.dubbing_id.ok_or_else(|| BuildError::missing_field("dubbing_id"))?,
            expected_duration_sec: self.expected_duration_sec.ok_or_else(|| BuildError::missing_field("expected_duration_sec"))?,
        })
    }
}
