pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// With diarize, two speaker_ids turned out to be one person: every earlier use of
/// from_speaker_id, in partial or diarized transcripts, is to_speaker_id. Ids are never
/// reused; merges chain, so apply them in the order received.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeakersMerged {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub from_speaker_id: String,
    #[serde(default)]
    pub to_speaker_id: String,
}

impl SpeakersMerged {
    pub fn builder() -> SpeakersMergedBuilder {
        <SpeakersMergedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakersMergedBuilder {
    message_type: Option<String>,
    from_speaker_id: Option<String>,
    to_speaker_id: Option<String>,
}

impl SpeakersMergedBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn from_speaker_id(mut self, value: impl Into<String>) -> Self {
        self.from_speaker_id = Some(value.into());
        self
    }

    pub fn to_speaker_id(mut self, value: impl Into<String>) -> Self {
        self.to_speaker_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeakersMerged`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_speaker_id`](SpeakersMergedBuilder::from_speaker_id)
    /// - [`to_speaker_id`](SpeakersMergedBuilder::to_speaker_id)
    pub fn build(self) -> Result<SpeakersMerged, BuildError> {
        Ok(SpeakersMerged {
            message_type: self.message_type,
            from_speaker_id: self.from_speaker_id.ok_or_else(|| BuildError::missing_field("from_speaker_id"))?,
            to_speaker_id: self.to_speaker_id.ok_or_else(|| BuildError::missing_field("to_speaker_id"))?,
        })
    }
}
