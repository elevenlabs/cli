pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExotelTransferOutput {
    pub config: ExotelTransferOutputConfig,
}

impl ExotelTransferOutput {
    pub fn builder() -> ExotelTransferOutputBuilder {
        <ExotelTransferOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelTransferOutputBuilder {
    config: Option<ExotelTransferOutputConfig>,
}

impl ExotelTransferOutputBuilder {
    pub fn config(mut self, value: ExotelTransferOutputConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExotelTransferOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ExotelTransferOutputBuilder::config)
    pub fn build(self) -> Result<ExotelTransferOutput, BuildError> {
        Ok(ExotelTransferOutput {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
