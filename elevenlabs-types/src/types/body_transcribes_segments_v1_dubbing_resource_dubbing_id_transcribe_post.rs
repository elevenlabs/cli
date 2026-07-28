pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePost {
    /// Transcribe this specific list of segments.
    #[serde(default)]
    pub segments: Vec<String>,
}

impl BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePost {
    pub fn builder() -> BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePostBuilder {
        <BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePostBuilder {
    segments: Option<Vec<String>>,
}

impl BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePostBuilder {
    pub fn segments(mut self, value: Vec<String>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePostBuilder::segments)
    pub fn build(self) -> Result<BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePost, BuildError> {
        Ok(BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePost {
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
        })
    }
}

