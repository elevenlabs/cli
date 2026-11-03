pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WhatsAppCallPermissionRequest {
    pub state: WhatsAppCallPermissionRequestState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responded_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_by_user_call: Option<bool>,
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
    responded_at_unix: Option<i64>,
    permanent: Option<bool>,
    granted_by_user_call: Option<bool>,
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

    pub fn responded_at_unix(mut self, value: i64) -> Self {
        self.responded_at_unix = Some(value);
        self
    }

    pub fn permanent(mut self, value: bool) -> Self {
        self.permanent = Some(value);
        self
    }

    pub fn granted_by_user_call(mut self, value: bool) -> Self {
        self.granted_by_user_call = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppCallPermissionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](WhatsAppCallPermissionRequestBuilder::state)
    pub fn build(self) -> Result<WhatsAppCallPermissionRequest, BuildError> {
        Ok(WhatsAppCallPermissionRequest {
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            requested_at_unix: self.requested_at_unix,
            responded_at_unix: self.responded_at_unix,
            permanent: self.permanent,
            granted_by_user_call: self.granted_by_user_call,
        })
    }
}
