pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PositionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub y: Option<f64>,
}

impl PositionInput {
    pub fn builder() -> PositionInputBuilder {
        <PositionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionInputBuilder {
    x: Option<f64>,
    y: Option<f64>,
}

impl PositionInputBuilder {
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PositionInput`].
    pub fn build(self) -> Result<PositionInput, BuildError> {
        Ok(PositionInput {
            x: self.x,
            y: self.y,
        })
    }
}
