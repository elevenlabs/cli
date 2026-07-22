pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthSettings {
    /// If set to true, starting a conversation with an agent will require a signed token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth: Option<bool>,
    /// A list of hosts that are allowed to start conversations with the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowlist: Option<Vec<AllowlistItem>>,
    /// When enabled, connections with no origin header will be rejected. If the allowlist is empty, this option has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_origin_header: Option<bool>,
    /// A shareable token that can be used to start a conversation with the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable_token: Option<String>,
}

impl AuthSettings {
    pub fn builder() -> AuthSettingsBuilder {
        <AuthSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthSettingsBuilder {
    enable_auth: Option<bool>,
    allowlist: Option<Vec<AllowlistItem>>,
    require_origin_header: Option<bool>,
    shareable_token: Option<String>,
}

impl AuthSettingsBuilder {
    pub fn enable_auth(mut self, value: bool) -> Self {
        self.enable_auth = Some(value);
        self
    }

    pub fn allowlist(mut self, value: Vec<AllowlistItem>) -> Self {
        self.allowlist = Some(value);
        self
    }

    pub fn require_origin_header(mut self, value: bool) -> Self {
        self.require_origin_header = Some(value);
        self
    }

    pub fn shareable_token(mut self, value: impl Into<String>) -> Self {
        self.shareable_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthSettings`].
    pub fn build(self) -> Result<AuthSettings, BuildError> {
        Ok(AuthSettings {
            enable_auth: self.enable_auth,
            allowlist: self.allowlist,
            require_origin_header: self.require_origin_header,
            shareable_token: self.shareable_token,
        })
    }
}
