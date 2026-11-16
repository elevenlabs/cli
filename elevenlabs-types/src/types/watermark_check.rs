pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WatermarkCheck {
    /// True when an ElevenLabs watermark was found.
    #[serde(default)]
    pub is_detected: bool,
    /// Matched generation record. Null when none matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<WatermarkDetails>,
}

impl WatermarkCheck {
    pub fn builder() -> WatermarkCheckBuilder {
        <WatermarkCheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WatermarkCheckBuilder {
    is_detected: Option<bool>,
    details: Option<WatermarkDetails>,
}

impl WatermarkCheckBuilder {
    pub fn is_detected(mut self, value: bool) -> Self {
        self.is_detected = Some(value);
        self
    }

    pub fn details(mut self, value: WatermarkDetails) -> Self {
        self.details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WatermarkCheck`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_detected`](WatermarkCheckBuilder::is_detected)
    pub fn build(self) -> Result<WatermarkCheck, BuildError> {
        Ok(WatermarkCheck {
            is_detected: self.is_detected.ok_or_else(|| BuildError::missing_field("is_detected"))?,
            details: self.details,
        })
    }
}
