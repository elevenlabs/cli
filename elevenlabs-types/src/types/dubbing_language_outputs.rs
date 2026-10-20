pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Signed, time-limited download URLs for a language target's outputs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingLanguageOutputs {
    /// Signed URL for the dubbed lossless audio track, in FLAC. The link expires one hour after it is issued; re-read the language target for a fresh one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lossless_audio: Option<String>,
    /// Signed URL for the translated lines as WebVTT subtitles, timed to the source speech. Null when none are available, such as for languages dubbed before subtitles were produced. The link expires one hour after it is issued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<String>,
}

impl DubbingLanguageOutputs {
    pub fn builder() -> DubbingLanguageOutputsBuilder {
        <DubbingLanguageOutputsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingLanguageOutputsBuilder {
    lossless_audio: Option<String>,
    subtitles: Option<String>,
}

impl DubbingLanguageOutputsBuilder {
    pub fn lossless_audio(mut self, value: impl Into<String>) -> Self {
        self.lossless_audio = Some(value.into());
        self
    }

    pub fn subtitles(mut self, value: impl Into<String>) -> Self {
        self.subtitles = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingLanguageOutputs`].
    pub fn build(self) -> Result<DubbingLanguageOutputs, BuildError> {
        Ok(DubbingLanguageOutputs {
            lossless_audio: self.lossless_audio,
            subtitles: self.subtitles,
        })
    }
}
