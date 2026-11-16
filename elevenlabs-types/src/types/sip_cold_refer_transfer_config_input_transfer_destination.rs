pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SipColdReferTransferConfigInputTransferDestination {
        #[serde(rename = "phone")]
        #[non_exhaustive]
        Phone {
            #[serde(flatten)]
            data: PhoneDestinationInput,
        },

        #[serde(rename = "phone_dynamic_variable")]
        #[non_exhaustive]
        PhoneDynamicVariable {
            #[serde(flatten)]
            data: PhoneDynamicVariableDestinationInput,
        },

        #[serde(rename = "sip_uri")]
        #[non_exhaustive]
        SipUri {
            #[serde(default)]
            destination: String,
        },

        #[serde(rename = "sip_uri_dynamic_variable")]
        #[non_exhaustive]
        SipUriDynamicVariable {
            #[serde(default)]
            destination: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SipColdReferTransferConfigInputTransferDestination {
    pub fn phone(data: PhoneDestinationInput) -> Self {
        Self::Phone { data }
    }

    pub fn phone_dynamic_variable(data: PhoneDynamicVariableDestinationInput) -> Self {
        Self::PhoneDynamicVariable { data }
    }

    pub fn sip_uri(destination: String) -> Self {
        Self::SipUri { destination }
    }

    pub fn sip_uri_dynamic_variable(destination: String) -> Self {
        Self::SipUriDynamicVariable { destination }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
