pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ExotelTransferInputConfig {
        #[serde(rename = "exotel_cold_blind_transfer")]
        #[non_exhaustive]
        ExotelColdBlindTransfer {
            transfer_destination: ExotelColdBlindTransferConfigInputTransferDestination,
        },

        #[serde(rename = "exotel_warm_transfer")]
        #[non_exhaustive]
        ExotelWarmTransfer {
            transfer_destination: ExotelWarmTransferConfigInputTransferDestination,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ExotelTransferInputConfig {
    pub fn exotel_cold_blind_transfer(transfer_destination: ExotelColdBlindTransferConfigInputTransferDestination) -> Self {
        Self::ExotelColdBlindTransfer { transfer_destination }
    }

    pub fn exotel_warm_transfer(transfer_destination: ExotelWarmTransferConfigInputTransferDestination) -> Self {
        Self::ExotelWarmTransfer { transfer_destination }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
