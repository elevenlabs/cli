pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExotelTransferInput {
    pub config: ExotelTransferInputConfig,
}

impl ExotelTransferInput {
    pub fn builder() -> ExotelTransferInputBuilder {
        <ExotelTransferInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelTransferInputBuilder {
    config: Option<ExotelTransferInputConfig>,
}

impl ExotelTransferInputBuilder {
    pub fn config(mut self, value: ExotelTransferInputConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExotelTransferInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ExotelTransferInputBuilder::config)
    pub fn build(self) -> Result<ExotelTransferInput, BuildError> {
        Ok(ExotelTransferInput {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
