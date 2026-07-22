pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPost {
    #[serde(default)]
    pub segment_ids: Vec<String>,
    #[serde(default)]
    pub speaker_id: String,
}

impl BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPost {
    pub fn builder() -> BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPostBuilder {
        <BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPostBuilder {
    segment_ids: Option<Vec<String>>,
    speaker_id: Option<String>,
}

impl BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPostBuilder {
    pub fn segment_ids(mut self, value: Vec<String>) -> Self {
        self.segment_ids = Some(value);
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segment_ids`](BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPostBuilder::segment_ids)
    /// - [`speaker_id`](BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPostBuilder::speaker_id)
    pub fn build(self) -> Result<BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPost, BuildError> {
        Ok(BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPost {
            segment_ids: self.segment_ids.ok_or_else(|| BuildError::missing_field("segment_ids"))?,
            speaker_id: self.speaker_id.ok_or_else(|| BuildError::missing_field("speaker_id"))?,
        })
    }
}

