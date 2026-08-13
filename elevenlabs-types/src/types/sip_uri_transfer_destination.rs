pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SipUriTransferDestination {
    #[serde(default)]
    pub sip_uri: String,
}

impl SipUriTransferDestination {
    pub fn builder() -> SipUriTransferDestinationBuilder {
        <SipUriTransferDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipUriTransferDestinationBuilder {
    sip_uri: Option<String>,
}

impl SipUriTransferDestinationBuilder {
    pub fn sip_uri(mut self, value: impl Into<String>) -> Self {
        self.sip_uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipUriTransferDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sip_uri`](SipUriTransferDestinationBuilder::sip_uri)
    pub fn build(self) -> Result<SipUriTransferDestination, BuildError> {
        Ok(SipUriTransferDestination {
            sip_uri: self.sip_uri.ok_or_else(|| BuildError::missing_field("sip_uri"))?,
        })
    }
}
