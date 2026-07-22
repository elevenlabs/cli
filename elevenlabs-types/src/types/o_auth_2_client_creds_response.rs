pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for oauth2 client creds
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OAuth2ClientCredsResponse {
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
    /// If True, send client credentials in Authorization header as Basic Auth instead of request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_in_header: Option<bool>,
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
    /// Custom headers configured for OAuth2 token requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<HashMap<String, String>>,
}

impl OAuth2ClientCredsResponse {
    pub fn builder() -> OAuth2ClientCredsResponseBuilder {
        <OAuth2ClientCredsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuth2ClientCredsResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    client_id: Option<String>,
    token_url: Option<String>,
    scopes: Option<Vec<String>>,
    extra_params: Option<HashMap<String, String>>,
    basic_auth_in_header: Option<bool>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
    custom_headers: Option<HashMap<String, String>>,
}

impl OAuth2ClientCredsResponseBuilder {
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

    pub fn basic_auth_in_header(mut self, value: bool) -> Self {
        self.basic_auth_in_header = Some(value);
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

    pub fn custom_headers(mut self, value: HashMap<String, String>) -> Self {
        self.custom_headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OAuth2ClientCredsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OAuth2ClientCredsResponseBuilder::name)
    /// - [`provider`](OAuth2ClientCredsResponseBuilder::provider)
    /// - [`client_id`](OAuth2ClientCredsResponseBuilder::client_id)
    /// - [`token_url`](OAuth2ClientCredsResponseBuilder::token_url)
    /// - [`id`](OAuth2ClientCredsResponseBuilder::id)
    pub fn build(self) -> Result<OAuth2ClientCredsResponse, BuildError> {
        Ok(OAuth2ClientCredsResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            token_url: self.token_url.ok_or_else(|| BuildError::missing_field("token_url"))?,
            scopes: self.scopes,
            extra_params: self.extra_params,
            basic_auth_in_header: self.basic_auth_in_header,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
            custom_headers: self.custom_headers,
        })
    }
}
