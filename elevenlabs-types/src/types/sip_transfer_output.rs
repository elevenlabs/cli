pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SipTransferOutput {
    pub config: SipTransferOutputConfig,
}

impl SipTransferOutput {
    pub fn builder() -> SipTransferOutputBuilder {
        <SipTransferOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipTransferOutputBuilder {
    config: Option<SipTransferOutputConfig>,
}

impl SipTransferOutputBuilder {
    pub fn config(mut self, value: SipTransferOutputConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipTransferOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](SipTransferOutputBuilder::config)
    pub fn build(self) -> Result<SipTransferOutput, BuildError> {
        Ok(SipTransferOutput {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
