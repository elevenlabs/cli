pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PodcastConversationModeData {
    /// The ID of the host voice.
    #[serde(default)]
    pub host_voice_id: String,
    /// The ID of the guest voice.
    #[serde(default)]
    pub guest_voice_id: String,
}

impl PodcastConversationModeData {
    pub fn builder() -> PodcastConversationModeDataBuilder {
        <PodcastConversationModeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastConversationModeDataBuilder {
    host_voice_id: Option<String>,
    guest_voice_id: Option<String>,
}

impl PodcastConversationModeDataBuilder {
    pub fn host_voice_id(mut self, value: impl Into<String>) -> Self {
        self.host_voice_id = Some(value.into());
        self
    }

    pub fn guest_voice_id(mut self, value: impl Into<String>) -> Self {
        self.guest_voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodcastConversationModeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`host_voice_id`](PodcastConversationModeDataBuilder::host_voice_id)
    /// - [`guest_voice_id`](PodcastConversationModeDataBuilder::guest_voice_id)
    pub fn build(self) -> Result<PodcastConversationModeData, BuildError> {
        Ok(PodcastConversationModeData {
            host_voice_id: self.host_voice_id.ok_or_else(|| BuildError::missing_field("host_voice_id"))?,
            guest_voice_id: self.guest_voice_id.ok_or_else(|| BuildError::missing_field("guest_voice_id"))?,
        })
    }
}
