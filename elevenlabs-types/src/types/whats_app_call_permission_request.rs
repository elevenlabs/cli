pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppCallPermissionRequest {
    pub state: WhatsAppCallPermissionRequestState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at_unix: Option<i64>,
}

impl WhatsAppCallPermissionRequest {
    pub fn builder() -> WhatsAppCallPermissionRequestBuilder {
        <WhatsAppCallPermissionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppCallPermissionRequestBuilder {
    state: Option<WhatsAppCallPermissionRequestState>,
    requested_at_unix: Option<i64>,
}

impl WhatsAppCallPermissionRequestBuilder {
    pub fn state(mut self, value: WhatsAppCallPermissionRequestState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn requested_at_unix(mut self, value: i64) -> Self {
        self.requested_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppCallPermissionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](WhatsAppCallPermissionRequestBuilder::state)
    pub fn build(self) -> Result<WhatsAppCallPermissionRequest, BuildError> {
        Ok(WhatsAppCallPermissionRequest {
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            requested_at_unix: self.requested_at_unix,
        })
    }
}
