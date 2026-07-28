pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationVoiceRewardModel {
    #[serde(default)]
    pub voice_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub reward_usd_cents: f64,
}

impl ConversationVoiceRewardModel {
    pub fn builder() -> ConversationVoiceRewardModelBuilder {
        <ConversationVoiceRewardModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationVoiceRewardModelBuilder {
    voice_id: Option<String>,
    reward_usd_cents: Option<f64>,
}

impl ConversationVoiceRewardModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn reward_usd_cents(mut self, value: f64) -> Self {
        self.reward_usd_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationVoiceRewardModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](ConversationVoiceRewardModelBuilder::voice_id)
    /// - [`reward_usd_cents`](ConversationVoiceRewardModelBuilder::reward_usd_cents)
    pub fn build(self) -> Result<ConversationVoiceRewardModel, BuildError> {
        Ok(ConversationVoiceRewardModel {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            reward_usd_cents: self.reward_usd_cents.ok_or_else(|| BuildError::missing_field("reward_usd_cents"))?,
        })
    }
}
