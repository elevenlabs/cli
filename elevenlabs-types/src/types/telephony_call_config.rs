pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TelephonyCallConfig {
    /// How long to ring the recipient before giving up, in seconds. Note that this will also be limited by the provider's own constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ringing_timeout_secs: Option<i64>,
    /// Whether to record the call using Twilio call recording. Ignored for non-Twilio providers. Recordings are stored in your Twilio account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twilio_call_recording_enabled: Option<bool>,
}

impl TelephonyCallConfig {
    pub fn builder() -> TelephonyCallConfigBuilder {
        <TelephonyCallConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelephonyCallConfigBuilder {
    ringing_timeout_secs: Option<i64>,
    twilio_call_recording_enabled: Option<bool>,
}

impl TelephonyCallConfigBuilder {
    pub fn ringing_timeout_secs(mut self, value: i64) -> Self {
        self.ringing_timeout_secs = Some(value);
        self
    }

    pub fn twilio_call_recording_enabled(mut self, value: bool) -> Self {
        self.twilio_call_recording_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TelephonyCallConfig`].
    pub fn build(self) -> Result<TelephonyCallConfig, BuildError> {
        Ok(TelephonyCallConfig {
            ringing_timeout_secs: self.ringing_timeout_secs,
            twilio_call_recording_enabled: self.twilio_call_recording_enabled,
        })
    }
}
