pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwilioTransferInput {
    pub config: TwilioTransferInputConfig,
}

impl TwilioTransferInput {
    pub fn builder() -> TwilioTransferInputBuilder {
        <TwilioTransferInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioTransferInputBuilder {
    config: Option<TwilioTransferInputConfig>,
}

impl TwilioTransferInputBuilder {
    pub fn config(mut self, value: TwilioTransferInputConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioTransferInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](TwilioTransferInputBuilder::config)
    pub fn build(self) -> Result<TwilioTransferInput, BuildError> {
        Ok(TwilioTransferInput {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
