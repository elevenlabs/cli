pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Per-agent (receptionist) configuration for client identity verification codes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionIdentityVerificationConfig {
    /// When enabled, the receptionist may only look up or modify a client's appointments and records after that client has completed OTP identity verification in this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<ReceptionNotificationChannel>>,
    /// Twilio phone number ID used to send SMS verification codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_phone_number_id: Option<String>,
}

impl ReceptionIdentityVerificationConfig {
    pub fn builder() -> ReceptionIdentityVerificationConfigBuilder {
        <ReceptionIdentityVerificationConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionIdentityVerificationConfigBuilder {
    enabled: Option<bool>,
    channels: Option<Vec<ReceptionNotificationChannel>>,
    sms_phone_number_id: Option<String>,
}

impl ReceptionIdentityVerificationConfigBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn channels(mut self, value: Vec<ReceptionNotificationChannel>) -> Self {
        self.channels = Some(value);
        self
    }

    pub fn sms_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.sms_phone_number_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionIdentityVerificationConfig`].
    pub fn build(self) -> Result<ReceptionIdentityVerificationConfig, BuildError> {
        Ok(ReceptionIdentityVerificationConfig {
            enabled: self.enabled,
            channels: self.channels,
            sms_phone_number_id: self.sms_phone_number_id,
        })
    }
}
