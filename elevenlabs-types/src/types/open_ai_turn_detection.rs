pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiTurnDetection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TurnDetectionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eagerness: Option<Eagerness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupt_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<i64>,
}

impl OpenAiTurnDetection {
    pub fn builder() -> OpenAiTurnDetectionBuilder {
        <OpenAiTurnDetectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiTurnDetectionBuilder {
    r#type: Option<TurnDetectionType>,
    eagerness: Option<Eagerness>,
    create_response: Option<bool>,
    interrupt_response: Option<bool>,
    threshold: Option<f64>,
    prefix_padding_ms: Option<i64>,
    silence_duration_ms: Option<i64>,
}

impl OpenAiTurnDetectionBuilder {
    pub fn r#type(mut self, value: TurnDetectionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn eagerness(mut self, value: Eagerness) -> Self {
        self.eagerness = Some(value);
        self
    }

    pub fn create_response(mut self, value: bool) -> Self {
        self.create_response = Some(value);
        self
    }

    pub fn interrupt_response(mut self, value: bool) -> Self {
        self.interrupt_response = Some(value);
        self
    }

    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
        self
    }

    pub fn prefix_padding_ms(mut self, value: i64) -> Self {
        self.prefix_padding_ms = Some(value);
        self
    }

    pub fn silence_duration_ms(mut self, value: i64) -> Self {
        self.silence_duration_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiTurnDetection`].
    pub fn build(self) -> Result<OpenAiTurnDetection, BuildError> {
        Ok(OpenAiTurnDetection {
            r#type: self.r#type,
            eagerness: self.eagerness,
            create_response: self.create_response,
            interrupt_response: self.interrupt_response,
            threshold: self.threshold,
            prefix_padding_ms: self.prefix_padding_ms,
            silence_duration_ms: self.silence_duration_ms,
        })
    }
}
