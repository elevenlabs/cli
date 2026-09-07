pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How to run Twilio's carrier-level answering machine detection (AMD) on a call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwilioMachineDetectionConfig {
    /// How thorough the detection should be. `enable` returns a verdict as soon as Twilio can tell a human from a machine. `detect_message_end` also waits for the voicemail greeting to finish, which is what produces the `machine_end_*` verdicts, but returns a result later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TwilioMachineDetectionMode>,
}

impl TwilioMachineDetectionConfig {
    pub fn builder() -> TwilioMachineDetectionConfigBuilder {
        <TwilioMachineDetectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioMachineDetectionConfigBuilder {
    mode: Option<TwilioMachineDetectionMode>,
}

impl TwilioMachineDetectionConfigBuilder {
    pub fn mode(mut self, value: TwilioMachineDetectionMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioMachineDetectionConfig`].
    pub fn build(self) -> Result<TwilioMachineDetectionConfig, BuildError> {
        Ok(TwilioMachineDetectionConfig {
            mode: self.mode,
        })
    }
}
