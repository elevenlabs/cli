pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Server payload containing an audio chunk for a specific context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioOutputMulti {
    /// Base64 encoded audio chunk.
    #[serde(default)]
    pub audio: String,
    #[serde(rename = "normalizedAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<NormalizedAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    /// The contextId for which this audio is.
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl AudioOutputMulti {
    pub fn builder() -> AudioOutputMultiBuilder {
        <AudioOutputMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioOutputMultiBuilder {
    audio: Option<String>,
    normalized_alignment: Option<NormalizedAlignment>,
    alignment: Option<Alignment>,
    context_id: Option<String>,
}

impl AudioOutputMultiBuilder {
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

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioOutputMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](AudioOutputMultiBuilder::audio)
    pub fn build(self) -> Result<AudioOutputMulti, BuildError> {
        Ok(AudioOutputMulti {
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
            normalized_alignment: self.normalized_alignment,
            alignment: self.alignment,
            context_id: self.context_id,
        })
    }
}
