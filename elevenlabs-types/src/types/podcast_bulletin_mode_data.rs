pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodcastBulletinModeData {
    /// The ID of the host voice.
    #[serde(default)]
    pub host_voice_id: String,
}

impl PodcastBulletinModeData {
    pub fn builder() -> PodcastBulletinModeDataBuilder {
        <PodcastBulletinModeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastBulletinModeDataBuilder {
    host_voice_id: Option<String>,
}

impl PodcastBulletinModeDataBuilder {
    pub fn host_voice_id(mut self, value: impl Into<String>) -> Self {
        self.host_voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodcastBulletinModeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`host_voice_id`](PodcastBulletinModeDataBuilder::host_voice_id)
    pub fn build(self) -> Result<PodcastBulletinModeData, BuildError> {
        Ok(PodcastBulletinModeData {
            host_voice_id: self.host_voice_id.ok_or_else(|| BuildError::missing_field("host_voice_id"))?,
        })
    }
}
