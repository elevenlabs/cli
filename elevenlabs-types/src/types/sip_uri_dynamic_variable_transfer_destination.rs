pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SipUriDynamicVariableTransferDestination {
    #[serde(default)]
    pub sip_uri: String,
}

impl SipUriDynamicVariableTransferDestination {
    pub fn builder() -> SipUriDynamicVariableTransferDestinationBuilder {
        <SipUriDynamicVariableTransferDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipUriDynamicVariableTransferDestinationBuilder {
    sip_uri: Option<String>,
}

impl SipUriDynamicVariableTransferDestinationBuilder {
    pub fn sip_uri(mut self, value: impl Into<String>) -> Self {
        self.sip_uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipUriDynamicVariableTransferDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sip_uri`](SipUriDynamicVariableTransferDestinationBuilder::sip_uri)
    pub fn build(self) -> Result<SipUriDynamicVariableTransferDestination, BuildError> {
        Ok(SipUriDynamicVariableTransferDestination {
            sip_uri: self.sip_uri.ok_or_else(|| BuildError::missing_field("sip_uri"))?,
        })
    }
}
