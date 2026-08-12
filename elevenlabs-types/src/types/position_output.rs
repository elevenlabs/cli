pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PositionOutput {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub x: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub y: f64,
}

impl PositionOutput {
    pub fn builder() -> PositionOutputBuilder {
        <PositionOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionOutputBuilder {
    x: Option<f64>,
    y: Option<f64>,
}

impl PositionOutputBuilder {
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PositionOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`x`](PositionOutputBuilder::x)
    /// - [`y`](PositionOutputBuilder::y)
    pub fn build(self) -> Result<PositionOutput, BuildError> {
        Ok(PositionOutput {
            x: self.x.ok_or_else(|| BuildError::missing_field("x"))?,
            y: self.y.ok_or_else(|| BuildError::missing_field("y"))?,
        })
    }
}
