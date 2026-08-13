pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SegmentUpdatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SegmentUpdatePayload {
    pub fn builder() -> SegmentUpdatePayloadBuilder {
        <SegmentUpdatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentUpdatePayloadBuilder {
    start_time: Option<f64>,
    end_time: Option<f64>,
    text: Option<String>,
}

impl SegmentUpdatePayloadBuilder {
    pub fn start_time(mut self, value: f64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: f64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SegmentUpdatePayload`].
    pub fn build(self) -> Result<SegmentUpdatePayload, BuildError> {
        Ok(SegmentUpdatePayload {
            start_time: self.start_time,
            end_time: self.end_time,
            text: self.text,
        })
    }
}

