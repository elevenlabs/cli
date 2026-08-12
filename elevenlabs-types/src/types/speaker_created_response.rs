pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeakerCreatedResponse {
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub speaker_id: String,
}

impl SpeakerCreatedResponse {
    pub fn builder() -> SpeakerCreatedResponseBuilder {
        <SpeakerCreatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerCreatedResponseBuilder {
    version: Option<i64>,
    speaker_id: Option<String>,
}

impl SpeakerCreatedResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeakerCreatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SpeakerCreatedResponseBuilder::version)
    /// - [`speaker_id`](SpeakerCreatedResponseBuilder::speaker_id)
    pub fn build(self) -> Result<SpeakerCreatedResponse, BuildError> {
        Ok(SpeakerCreatedResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            speaker_id: self.speaker_id.ok_or_else(|| BuildError::missing_field("speaker_id"))?,
        })
    }
}
