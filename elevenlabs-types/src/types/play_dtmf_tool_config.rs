pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Allows the agent to play DTMF tones during a phone call.
/// 
/// This tool can be used to interact with automated phone systems, such as
/// navigating phone menus, entering extensions, or inputting numeric codes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlayDtmfToolConfig {
    /// Send DTMF tones as out-of-band RTP events (RFC 4733) instead of in-band audio. Only effective for SIP trunk imported numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_out_of_band_dtmf: Option<bool>,
    /// If true, the agent will not generate further speech after playing DTMF tones. This prevents the agent's speech from interfering with IVR systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_turn_after_dtmf: Option<bool>,
}

impl PlayDtmfToolConfig {
    pub fn builder() -> PlayDtmfToolConfigBuilder {
        <PlayDtmfToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlayDtmfToolConfigBuilder {
    use_out_of_band_dtmf: Option<bool>,
    suppress_turn_after_dtmf: Option<bool>,
}

impl PlayDtmfToolConfigBuilder {
    pub fn use_out_of_band_dtmf(mut self, value: bool) -> Self {
        self.use_out_of_band_dtmf = Some(value);
        self
    }

    pub fn suppress_turn_after_dtmf(mut self, value: bool) -> Self {
        self.suppress_turn_after_dtmf = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlayDtmfToolConfig`].
    pub fn build(self) -> Result<PlayDtmfToolConfig, BuildError> {
        Ok(PlayDtmfToolConfig {
            use_out_of_band_dtmf: self.use_out_of_band_dtmf,
            suppress_turn_after_dtmf: self.suppress_turn_after_dtmf,
        })
    }
}
