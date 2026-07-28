pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SegmentSubtitleFrame {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_time: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_time: f64,
    #[serde(default)]
    pub lines: Vec<String>,
}

impl SegmentSubtitleFrame {
    pub fn builder() -> SegmentSubtitleFrameBuilder {
        <SegmentSubtitleFrameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentSubtitleFrameBuilder {
    start_time: Option<f64>,
    end_time: Option<f64>,
    lines: Option<Vec<String>>,
}

impl SegmentSubtitleFrameBuilder {
    pub fn start_time(mut self, value: f64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: f64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<String>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentSubtitleFrame`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](SegmentSubtitleFrameBuilder::start_time)
    /// - [`end_time`](SegmentSubtitleFrameBuilder::end_time)
    /// - [`lines`](SegmentSubtitleFrameBuilder::lines)
    pub fn build(self) -> Result<SegmentSubtitleFrame, BuildError> {
        Ok(SegmentSubtitleFrame {
            start_time: self.start_time.ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self.end_time.ok_or_else(|| BuildError::missing_field("end_time"))?,
            lines: self.lines.ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
