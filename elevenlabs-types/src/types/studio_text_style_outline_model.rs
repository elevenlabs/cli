pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StudioTextStyleOutlineModel {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub opacity: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub width: f64,
}

impl StudioTextStyleOutlineModel {
    pub fn builder() -> StudioTextStyleOutlineModelBuilder {
        <StudioTextStyleOutlineModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioTextStyleOutlineModelBuilder {
    enabled: Option<bool>,
    color: Option<String>,
    opacity: Option<f64>,
    width: Option<f64>,
}

impl StudioTextStyleOutlineModelBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn opacity(mut self, value: f64) -> Self {
        self.opacity = Some(value);
        self
    }

    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StudioTextStyleOutlineModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](StudioTextStyleOutlineModelBuilder::enabled)
    /// - [`color`](StudioTextStyleOutlineModelBuilder::color)
    /// - [`opacity`](StudioTextStyleOutlineModelBuilder::opacity)
    /// - [`width`](StudioTextStyleOutlineModelBuilder::width)
    pub fn build(self) -> Result<StudioTextStyleOutlineModel, BuildError> {
        Ok(StudioTextStyleOutlineModel {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            color: self.color.ok_or_else(|| BuildError::missing_field("color"))?,
            opacity: self.opacity.ok_or_else(|| BuildError::missing_field("opacity"))?,
            width: self.width.ok_or_else(|| BuildError::missing_field("width"))?,
        })
    }
}
