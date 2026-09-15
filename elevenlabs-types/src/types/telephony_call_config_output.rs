pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TelephonyCallConfigOutput {
    /// How long to ring the recipient before giving up, in seconds. Note that this will also be limited by the provider's own constraints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ringing_timeout_secs: Option<i64>,
    /// Whether to record the call using Twilio call recording. Ignored for non-Twilio providers. Recordings are stored in your Twilio account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twilio_call_recording_enabled: Option<bool>,
    /// Configuration for Twilio's carrier-level answering machine detection (AMD). Omit or set to null to disable it. Ignored for non-Twilio providers and for inbound calls. The resulting verdict is delivered as its own `answering_machine_detection` webhook event, which requires that event to be enabled on the workspace or agent webhook settings; it is not part of the conversation or the post-call webhook. Detection runs asynchronously so it never delays the start of the conversation, and the verdict can arrive at any point during the call -- with `detect_message_end`, even after it has ended. Twilio bills separately for AMD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twilio_machine_detection: Option<TwilioMachineDetectionConfig>,
}

impl TelephonyCallConfigOutput {
    pub fn builder() -> TelephonyCallConfigOutputBuilder {
        <TelephonyCallConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelephonyCallConfigOutputBuilder {
    ringing_timeout_secs: Option<i64>,
    twilio_call_recording_enabled: Option<bool>,
    twilio_machine_detection: Option<TwilioMachineDetectionConfig>,
}

impl TelephonyCallConfigOutputBuilder {
    pub fn ringing_timeout_secs(mut self, value: i64) -> Self {
        self.ringing_timeout_secs = Some(value);
        self
    }

    pub fn twilio_call_recording_enabled(mut self, value: bool) -> Self {
        self.twilio_call_recording_enabled = Some(value);
        self
    }

    pub fn twilio_machine_detection(mut self, value: TwilioMachineDetectionConfig) -> Self {
        self.twilio_machine_detection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TelephonyCallConfigOutput`].
    pub fn build(self) -> Result<TelephonyCallConfigOutput, BuildError> {
        Ok(TelephonyCallConfigOutput {
            ringing_timeout_secs: self.ringing_timeout_secs,
            twilio_call_recording_enabled: self.twilio_call_recording_enabled,
            twilio_machine_detection: self.twilio_machine_detection,
        })
    }
}
