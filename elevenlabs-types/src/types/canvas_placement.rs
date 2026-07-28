pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Defines asset positioning and transformation on canvas.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CanvasPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub x_relative: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub y_relative: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub scale_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub scale_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pivot_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pivot_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub skew_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub skew_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub crop_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub crop_right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub crop_bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub crop_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_x: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_y: Option<bool>,
}

impl CanvasPlacement {
    pub fn builder() -> CanvasPlacementBuilder {
        <CanvasPlacementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CanvasPlacementBuilder {
    x_relative: Option<f64>,
    y_relative: Option<f64>,
    scale_x: Option<f64>,
    scale_y: Option<f64>,
    pivot_x: Option<f64>,
    pivot_y: Option<f64>,
    skew_x: Option<f64>,
    skew_y: Option<f64>,
    crop_top: Option<f64>,
    crop_right: Option<f64>,
    crop_bottom: Option<f64>,
    crop_left: Option<f64>,
    flip_x: Option<bool>,
    flip_y: Option<bool>,
}

impl CanvasPlacementBuilder {
    pub fn x_relative(mut self, value: f64) -> Self {
        self.x_relative = Some(value);
        self
    }

    pub fn y_relative(mut self, value: f64) -> Self {
        self.y_relative = Some(value);
        self
    }

    pub fn scale_x(mut self, value: f64) -> Self {
        self.scale_x = Some(value);
        self
    }

    pub fn scale_y(mut self, value: f64) -> Self {
        self.scale_y = Some(value);
        self
    }

    pub fn pivot_x(mut self, value: f64) -> Self {
        self.pivot_x = Some(value);
        self
    }

    pub fn pivot_y(mut self, value: f64) -> Self {
        self.pivot_y = Some(value);
        self
    }

    pub fn skew_x(mut self, value: f64) -> Self {
        self.skew_x = Some(value);
        self
    }

    pub fn skew_y(mut self, value: f64) -> Self {
        self.skew_y = Some(value);
        self
    }

    pub fn crop_top(mut self, value: f64) -> Self {
        self.crop_top = Some(value);
        self
    }

    pub fn crop_right(mut self, value: f64) -> Self {
        self.crop_right = Some(value);
        self
    }

    pub fn crop_bottom(mut self, value: f64) -> Self {
        self.crop_bottom = Some(value);
        self
    }

    pub fn crop_left(mut self, value: f64) -> Self {
        self.crop_left = Some(value);
        self
    }

    pub fn flip_x(mut self, value: bool) -> Self {
        self.flip_x = Some(value);
        self
    }

    pub fn flip_y(mut self, value: bool) -> Self {
        self.flip_y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CanvasPlacement`].
    pub fn build(self) -> Result<CanvasPlacement, BuildError> {
        Ok(CanvasPlacement {
            x_relative: self.x_relative,
            y_relative: self.y_relative,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            pivot_x: self.pivot_x,
            pivot_y: self.pivot_y,
            skew_x: self.skew_x,
            skew_y: self.skew_y,
            crop_top: self.crop_top,
            crop_right: self.crop_right,
            crop_bottom: self.crop_bottom,
            crop_left: self.crop_left,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
        })
    }
}
