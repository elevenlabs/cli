pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SmsConversationInfo {
    pub direction: SmsConversationInfoDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    #[serde(default)]
    pub sms_user_phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_phone_number: Option<String>,
}

impl SmsConversationInfo {
    pub fn builder() -> SmsConversationInfoBuilder {
        <SmsConversationInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmsConversationInfoBuilder {
    direction: Option<SmsConversationInfoDirection>,
    phone_number_id: Option<String>,
    sms_user_phone_number: Option<String>,
    agent_phone_number: Option<String>,
}

impl SmsConversationInfoBuilder {
    pub fn direction(mut self, value: SmsConversationInfoDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn sms_user_phone_number(mut self, value: impl Into<String>) -> Self {
        self.sms_user_phone_number = Some(value.into());
        self
    }

    pub fn agent_phone_number(mut self, value: impl Into<String>) -> Self {
        self.agent_phone_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SmsConversationInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](SmsConversationInfoBuilder::direction)
    /// - [`sms_user_phone_number`](SmsConversationInfoBuilder::sms_user_phone_number)
    pub fn build(self) -> Result<SmsConversationInfo, BuildError> {
        Ok(SmsConversationInfo {
            direction: self.direction.ok_or_else(|| BuildError::missing_field("direction"))?,
            phone_number_id: self.phone_number_id,
            sms_user_phone_number: self.sms_user_phone_number.ok_or_else(|| BuildError::missing_field("sms_user_phone_number"))?,
            agent_phone_number: self.agent_phone_number,
        })
    }
}
