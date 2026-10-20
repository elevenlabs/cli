pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiAudioInputTranscription {
    /// Language code of the conversation (e.g. en, es, pt-br), used for transcription, speech and the model's reply language. Defaults to en.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl OpenAiAudioInputTranscription {
    pub fn builder() -> OpenAiAudioInputTranscriptionBuilder {
        <OpenAiAudioInputTranscriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioInputTranscriptionBuilder {
    language: Option<String>,
}

impl OpenAiAudioInputTranscriptionBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioInputTranscription`].
    pub fn build(self) -> Result<OpenAiAudioInputTranscription, BuildError> {
        Ok(OpenAiAudioInputTranscription {
            language: self.language,
        })
    }
}
