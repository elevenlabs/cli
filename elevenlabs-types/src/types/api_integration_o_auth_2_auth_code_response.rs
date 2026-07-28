pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for integration-managed OAuth2 Auth Code auth connections
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiIntegrationOAuth2AuthCodeResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Separator for scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_separator: Option<ApiIntegrationOAuth2AuthCodeResponseScopeSeparator>,
    /// ISO 8601 timestamp of when the access token expires
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub integration_id: String,
    #[serde(default)]
    pub credential_id: String,
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

impl ApiIntegrationOAuth2AuthCodeResponse {
    pub fn builder() -> ApiIntegrationOAuth2AuthCodeResponseBuilder {
        <ApiIntegrationOAuth2AuthCodeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiIntegrationOAuth2AuthCodeResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    token_url: Option<String>,
    scopes: Option<Vec<String>>,
    scope_separator: Option<ApiIntegrationOAuth2AuthCodeResponseScopeSeparator>,
    expires_at: Option<String>,
    integration_id: Option<String>,
    credential_id: Option<String>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl ApiIntegrationOAuth2AuthCodeResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
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

    pub fn scope_separator(mut self, value: ApiIntegrationOAuth2AuthCodeResponseScopeSeparator) -> Self {
        self.scope_separator = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ApiIntegrationOAuth2AuthCodeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ApiIntegrationOAuth2AuthCodeResponseBuilder::name)
    /// - [`provider`](ApiIntegrationOAuth2AuthCodeResponseBuilder::provider)
    /// - [`token_url`](ApiIntegrationOAuth2AuthCodeResponseBuilder::token_url)
    /// - [`expires_at`](ApiIntegrationOAuth2AuthCodeResponseBuilder::expires_at)
    /// - [`integration_id`](ApiIntegrationOAuth2AuthCodeResponseBuilder::integration_id)
    /// - [`credential_id`](ApiIntegrationOAuth2AuthCodeResponseBuilder::credential_id)
    /// - [`id`](ApiIntegrationOAuth2AuthCodeResponseBuilder::id)
    pub fn build(self) -> Result<ApiIntegrationOAuth2AuthCodeResponse, BuildError> {
        Ok(ApiIntegrationOAuth2AuthCodeResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            token_url: self.token_url.ok_or_else(|| BuildError::missing_field("token_url"))?,
            scopes: self.scopes,
            scope_separator: self.scope_separator,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            integration_id: self.integration_id.ok_or_else(|| BuildError::missing_field("integration_id"))?,
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
