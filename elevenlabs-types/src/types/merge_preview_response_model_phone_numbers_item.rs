pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "provider")]
#[non_exhaustive]
pub enum MergePreviewResponseModelPhoneNumbersItem {
        #[serde(rename = "exotel")]
        #[non_exhaustive]
        Exotel {
            #[serde(flatten)]
            data: GetPhoneNumberExotelResponseModel,
        },

        #[serde(rename = "sip_trunk")]
        #[non_exhaustive]
        SipTrunk {
            #[serde(flatten)]
            data: GetPhoneNumberSipTrunkResponseModel,
        },

        #[serde(rename = "twilio")]
        #[non_exhaustive]
        Twilio {
            #[serde(flatten)]
            data: GetPhoneNumberTwilioResponseModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl MergePreviewResponseModelPhoneNumbersItem {
    pub fn exotel(data: GetPhoneNumberExotelResponseModel) -> Self {
        Self::Exotel { data }
    }

    pub fn sip_trunk(data: GetPhoneNumberSipTrunkResponseModel) -> Self {
        Self::SipTrunk { data }
    }

    pub fn twilio(data: GetPhoneNumberTwilioResponseModel) -> Self {
        Self::Twilio { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
