pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowPhoneNumberNodeModelInputTransferDestination {
        #[serde(rename = "phone")]
        #[non_exhaustive]
        Phone {
            #[serde(flatten)]
            data: PhoneNumberTransferDestination,
        },

        #[serde(rename = "phone_dynamic_variable")]
        #[non_exhaustive]
        PhoneDynamicVariable {
            #[serde(flatten)]
            data: PhoneNumberDynamicVariableTransferDestination,
        },

        #[serde(rename = "sip_uri")]
        #[non_exhaustive]
        SipUri {
            #[serde(flatten)]
            data: SipUriTransferDestination,
        },

        #[serde(rename = "sip_uri_dynamic_variable")]
        #[non_exhaustive]
        SipUriDynamicVariable {
            #[serde(flatten)]
            data: SipUriDynamicVariableTransferDestination,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WorkflowPhoneNumberNodeModelInputTransferDestination {
    pub fn phone(data: PhoneNumberTransferDestination) -> Self {
        Self::Phone { data }
    }

    pub fn phone_dynamic_variable(data: PhoneNumberDynamicVariableTransferDestination) -> Self {
        Self::PhoneDynamicVariable { data }
    }

    pub fn sip_uri(data: SipUriTransferDestination) -> Self {
        Self::SipUri { data }
    }

    pub fn sip_uri_dynamic_variable(data: SipUriDynamicVariableTransferDestination) -> Self {
        Self::SipUriDynamicVariable { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
