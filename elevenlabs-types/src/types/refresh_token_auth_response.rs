pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for OAuth2 refresh-token-grant auth connections
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefreshTokenAuthResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, String>>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<AuthConnectionDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AuthConnectionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_updated_at: Option<String>,
}

impl RefreshTokenAuthResponse {
    pub fn builder() -> RefreshTokenAuthResponseBuilder {
        <RefreshTokenAuthResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefreshTokenAuthResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    client_id: Option<String>,
    token_url: Option<String>,
    scopes: Option<Vec<String>>,
    extra_params: Option<HashMap<String, String>>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl RefreshTokenAuthResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn token_url(mut self, value: impl Into<String>) -> Self {
        self.token_url = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn extra_params(mut self, value: HashMap<String, String>) -> Self {
        self.extra_params = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn used_by(mut self, value: AuthConnectionDependencies) -> Self {
        self.used_by = Some(value);
        self
    }

    pub fn status(mut self, value: AuthConnectionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_detail(mut self, value: impl Into<String>) -> Self {
        self.status_detail = Some(value.into());
        self
    }

    pub fn status_updated_at(mut self, value: impl Into<String>) -> Self {
        self.status_updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefreshTokenAuthResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RefreshTokenAuthResponseBuilder::name)
    /// - [`provider`](RefreshTokenAuthResponseBuilder::provider)
    /// - [`client_id`](RefreshTokenAuthResponseBuilder::client_id)
    /// - [`token_url`](RefreshTokenAuthResponseBuilder::token_url)
    /// - [`id`](RefreshTokenAuthResponseBuilder::id)
    pub fn build(self) -> Result<RefreshTokenAuthResponse, BuildError> {
        Ok(RefreshTokenAuthResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            token_url: self.token_url.ok_or_else(|| BuildError::missing_field("token_url"))?,
            scopes: self.scopes,
            extra_params: self.extra_params,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
