pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TwilioTransferOutput {
    pub config: TwilioTransferOutputConfig,
}

impl TwilioTransferOutput {
    pub fn builder() -> TwilioTransferOutputBuilder {
        <TwilioTransferOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioTransferOutputBuilder {
    config: Option<TwilioTransferOutputConfig>,
}

impl TwilioTransferOutputBuilder {
    pub fn config(mut self, value: TwilioTransferOutputConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioTransferOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](TwilioTransferOutputBuilder::config)
    pub fn build(self) -> Result<TwilioTransferOutput, BuildError> {
        Ok(TwilioTransferOutput {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
