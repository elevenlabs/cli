pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioOutput {
    /// A generated partial audio chunk, encoded using the selected output_format, by default this
    /// is MP3 encoded as a base64 string.
    #[serde(default)]
    pub audio: String,
    #[serde(rename = "normalizedAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<NormalizedAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
}

impl AudioOutput {
    pub fn builder() -> AudioOutputBuilder {
        <AudioOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioOutputBuilder {
    audio: Option<String>,
    normalized_alignment: Option<NormalizedAlignment>,
    alignment: Option<Alignment>,
}

impl AudioOutputBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn normalized_alignment(mut self, value: NormalizedAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    pub fn alignment(mut self, value: Alignment) -> Self {
        self.alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](AudioOutputBuilder::audio)
    pub fn build(self) -> Result<AudioOutput, BuildError> {
        Ok(AudioOutput {
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
            normalized_alignment: self.normalized_alignment,
            alignment: self.alignment,
        })
    }
}
