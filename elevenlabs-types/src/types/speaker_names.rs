pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// With infer_speaker_names, the names inferred so far for diarized speakers, keyed
/// by speaker_id. Sent in full each time a speaker is named or a merge changes the
/// names.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeakerNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub speaker_names: HashMap<String, String>,
}

impl SpeakerNames {
    pub fn builder() -> SpeakerNamesBuilder {
        <SpeakerNamesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerNamesBuilder {
    message_type: Option<String>,
    speaker_names: Option<HashMap<String, String>>,
}

impl SpeakerNamesBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn speaker_names(mut self, value: HashMap<String, String>) -> Self {
        self.speaker_names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeakerNames`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speaker_names`](SpeakerNamesBuilder::speaker_names)
    pub fn build(self) -> Result<SpeakerNames, BuildError> {
        Ok(SpeakerNames {
            message_type: self.message_type,
            speaker_names: self.speaker_names.ok_or_else(|| BuildError::missing_field("speaker_names"))?,
        })
    }
}
