pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StudioTextStyleShadowModel {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub opacity: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub blur: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset_x: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub offset_y: f64,
}

impl StudioTextStyleShadowModel {
    pub fn builder() -> StudioTextStyleShadowModelBuilder {
        <StudioTextStyleShadowModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioTextStyleShadowModelBuilder {
    enabled: Option<bool>,
    color: Option<String>,
    opacity: Option<f64>,
    blur: Option<f64>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
}

impl StudioTextStyleShadowModelBuilder {
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

    pub fn blur(mut self, value: f64) -> Self {
        self.blur = Some(value);
        self
    }

    pub fn offset_x(mut self, value: f64) -> Self {
        self.offset_x = Some(value);
        self
    }

    pub fn offset_y(mut self, value: f64) -> Self {
        self.offset_y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StudioTextStyleShadowModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](StudioTextStyleShadowModelBuilder::enabled)
    /// - [`color`](StudioTextStyleShadowModelBuilder::color)
    /// - [`opacity`](StudioTextStyleShadowModelBuilder::opacity)
    /// - [`blur`](StudioTextStyleShadowModelBuilder::blur)
    /// - [`offset_x`](StudioTextStyleShadowModelBuilder::offset_x)
    /// - [`offset_y`](StudioTextStyleShadowModelBuilder::offset_y)
    pub fn build(self) -> Result<StudioTextStyleShadowModel, BuildError> {
        Ok(StudioTextStyleShadowModel {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            color: self.color.ok_or_else(|| BuildError::missing_field("color"))?,
            opacity: self.opacity.ok_or_else(|| BuildError::missing_field("opacity"))?,
            blur: self.blur.ok_or_else(|| BuildError::missing_field("blur"))?,
            offset_x: self.offset_x.ok_or_else(|| BuildError::missing_field("offset_x"))?,
            offset_y: self.offset_y.ok_or_else(|| BuildError::missing_field("offset_y"))?,
        })
    }
}
