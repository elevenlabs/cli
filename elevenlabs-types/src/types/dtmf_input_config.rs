pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configuration for DTMF (keypad) input collection during phone calls.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DtmfInputConfig {
    /// Timeout in seconds to wait for additional DTMF digits
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dtmf_input_timeout: Option<f64>,
    /// If true, pressing # immediately completes DTMF input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_terminator: Option<bool>,
    /// If true, replace the caller's DTMF (keypad) entries with a redaction marker in the transcript, conversation log and analysis. Digits the agent repeats back or passes to a tool are not affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_input: Option<bool>,
}

impl DtmfInputConfig {
    pub fn builder() -> DtmfInputConfigBuilder {
        <DtmfInputConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DtmfInputConfigBuilder {
    dtmf_input_timeout: Option<f64>,
    hash_terminator: Option<bool>,
    redact_input: Option<bool>,
}

impl DtmfInputConfigBuilder {
    pub fn dtmf_input_timeout(mut self, value: f64) -> Self {
        self.dtmf_input_timeout = Some(value);
        self
    }

    pub fn hash_terminator(mut self, value: bool) -> Self {
        self.hash_terminator = Some(value);
        self
    }

    pub fn redact_input(mut self, value: bool) -> Self {
        self.redact_input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DtmfInputConfig`].
    pub fn build(self) -> Result<DtmfInputConfig, BuildError> {
        Ok(DtmfInputConfig {
            dtmf_input_timeout: self.dtmf_input_timeout,
            hash_terminator: self.hash_terminator,
            redact_input: self.redact_input,
        })
    }
}
