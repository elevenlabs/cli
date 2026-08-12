pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost {
    /// Whether to convert the audio to mpeg format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_to_mpeg: Option<bool>,
}

impl BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost {
    pub fn builder() -> BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder {
        <BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder {
    convert_to_mpeg: Option<bool>,
}

impl BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPostBuilder {
    pub fn convert_to_mpeg(mut self, value: bool) -> Self {
        self.convert_to_mpeg = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost`].
    pub fn build(self) -> Result<BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost, BuildError> {
        Ok(BodyStreamStudioProjectAudioV1StudioProjectsProjectIdSnapshotsProjectSnapshotIdStreamPost {
            convert_to_mpeg: self.convert_to_mpeg,
        })
    }
}

