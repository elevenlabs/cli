pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaptionStyleVerticalPlacementModel {
    pub align: CaptionStyleVerticalPlacementModelAlign,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub translate_pct: f64,
}

impl CaptionStyleVerticalPlacementModel {
    pub fn builder() -> CaptionStyleVerticalPlacementModelBuilder {
        <CaptionStyleVerticalPlacementModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleVerticalPlacementModelBuilder {
    align: Option<CaptionStyleVerticalPlacementModelAlign>,
    translate_pct: Option<f64>,
}

impl CaptionStyleVerticalPlacementModelBuilder {
    pub fn align(mut self, value: CaptionStyleVerticalPlacementModelAlign) -> Self {
        self.align = Some(value);
        self
    }

    pub fn translate_pct(mut self, value: f64) -> Self {
        self.translate_pct = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleVerticalPlacementModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`align`](CaptionStyleVerticalPlacementModelBuilder::align)
    /// - [`translate_pct`](CaptionStyleVerticalPlacementModelBuilder::translate_pct)
    pub fn build(self) -> Result<CaptionStyleVerticalPlacementModel, BuildError> {
        Ok(CaptionStyleVerticalPlacementModel {
            align: self.align.ok_or_else(|| BuildError::missing_field("align"))?,
            translate_pct: self.translate_pct.ok_or_else(|| BuildError::missing_field("translate_pct"))?,
        })
    }
}
