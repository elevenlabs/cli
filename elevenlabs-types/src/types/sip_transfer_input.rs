pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SipTransferInput {
    pub config: SipTransferInputConfig,
}

impl SipTransferInput {
    pub fn builder() -> SipTransferInputBuilder {
        <SipTransferInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipTransferInputBuilder {
    config: Option<SipTransferInputConfig>,
}

impl SipTransferInputBuilder {
    pub fn config(mut self, value: SipTransferInputConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipTransferInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](SipTransferInputBuilder::config)
    pub fn build(self) -> Result<SipTransferInput, BuildError> {
        Ok(SipTransferInput {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
