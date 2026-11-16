pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ExotelTransferOutputConfig {
        #[serde(rename = "exotel_cold_blind_transfer")]
        #[non_exhaustive]
        ExotelColdBlindTransfer {
            transfer_destination: ExotelColdBlindTransferConfigOutputTransferDestination,
        },

        #[serde(rename = "exotel_warm_transfer")]
        #[non_exhaustive]
        ExotelWarmTransfer {
            transfer_destination: ExotelWarmTransferConfigOutputTransferDestination,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ExotelTransferOutputConfig {
    pub fn exotel_cold_blind_transfer(transfer_destination: ExotelColdBlindTransferConfigOutputTransferDestination) -> Self {
        Self::ExotelColdBlindTransfer { transfer_destination }
    }

    pub fn exotel_warm_transfer(transfer_destination: ExotelWarmTransferConfigOutputTransferDestination) -> Self {
        Self::ExotelWarmTransfer { transfer_destination }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
