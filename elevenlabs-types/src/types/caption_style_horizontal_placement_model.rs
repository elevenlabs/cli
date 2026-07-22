pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaptionStyleHorizontalPlacementModel {
    pub align: CaptionStyleHorizontalPlacementModelAlign,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub translate_pct: f64,
}

impl CaptionStyleHorizontalPlacementModel {
    pub fn builder() -> CaptionStyleHorizontalPlacementModelBuilder {
        <CaptionStyleHorizontalPlacementModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleHorizontalPlacementModelBuilder {
    align: Option<CaptionStyleHorizontalPlacementModelAlign>,
    translate_pct: Option<f64>,
}

impl CaptionStyleHorizontalPlacementModelBuilder {
    pub fn align(mut self, value: CaptionStyleHorizontalPlacementModelAlign) -> Self {
        self.align = Some(value);
        self
    }

    pub fn translate_pct(mut self, value: f64) -> Self {
        self.translate_pct = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleHorizontalPlacementModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`align`](CaptionStyleHorizontalPlacementModelBuilder::align)
    /// - [`translate_pct`](CaptionStyleHorizontalPlacementModelBuilder::translate_pct)
    pub fn build(self) -> Result<CaptionStyleHorizontalPlacementModel, BuildError> {
        Ok(CaptionStyleHorizontalPlacementModel {
            align: self.align.ok_or_else(|| BuildError::missing_field("align"))?,
            translate_pct: self.translate_pct.ok_or_else(|| BuildError::missing_field("translate_pct"))?,
        })
    }
}
