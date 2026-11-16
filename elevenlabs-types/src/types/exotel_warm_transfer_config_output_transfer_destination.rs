pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ExotelWarmTransferConfigOutputTransferDestination {
        #[serde(rename = "phone")]
        #[non_exhaustive]
        Phone {
            #[serde(flatten)]
            data: PhoneDestinationOutput,
        },

        #[serde(rename = "phone_dynamic_variable")]
        #[non_exhaustive]
        PhoneDynamicVariable {
            #[serde(flatten)]
            data: PhoneDynamicVariableDestinationOutput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ExotelWarmTransferConfigOutputTransferDestination {
    pub fn phone(data: PhoneDestinationOutput) -> Self {
        Self::Phone { data }
    }

    pub fn phone_dynamic_variable(data: PhoneDynamicVariableDestinationOutput) -> Self {
        Self::PhoneDynamicVariable { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
