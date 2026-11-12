pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingAudio {
    pub message_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub sample_rate: i64,
}

impl DubbingAudio {
    pub fn builder() -> DubbingAudioBuilder {
        <DubbingAudioBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingAudioBuilder {
    message_type: Option<String>,
    data: Option<String>,
    sample_rate: Option<i64>,
}

impl DubbingAudioBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingAudio`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](DubbingAudioBuilder::message_type)
    /// - [`data`](DubbingAudioBuilder::data)
    /// - [`sample_rate`](DubbingAudioBuilder::sample_rate)
    pub fn build(self) -> Result<DubbingAudio, BuildError> {
        Ok(DubbingAudio {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            sample_rate: self.sample_rate.ok_or_else(|| BuildError::missing_field("sample_rate"))?,
        })
    }
}
