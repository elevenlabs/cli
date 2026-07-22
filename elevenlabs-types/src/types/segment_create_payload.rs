pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SegmentCreatePayload {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_time: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, Option<String>>>,
}

impl SegmentCreatePayload {
    pub fn builder() -> SegmentCreatePayloadBuilder {
        <SegmentCreatePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentCreatePayloadBuilder {
    start_time: Option<f64>,
    end_time: Option<f64>,
    text: Option<String>,
    translations: Option<HashMap<String, Option<String>>>,
}

impl SegmentCreatePayloadBuilder {
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

    pub fn translations(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentCreatePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](SegmentCreatePayloadBuilder::start_time)
    /// - [`end_time`](SegmentCreatePayloadBuilder::end_time)
    pub fn build(self) -> Result<SegmentCreatePayload, BuildError> {
        Ok(SegmentCreatePayload {
            start_time: self.start_time.ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self.end_time.ok_or_else(|| BuildError::missing_field("end_time"))?,
            text: self.text,
            translations: self.translations,
        })
    }
}

