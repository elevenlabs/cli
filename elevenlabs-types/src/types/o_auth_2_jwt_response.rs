pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for OAuth2 JWT auth connections
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OAuth2JwtResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    /// JWT signing algorithm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<OAuth2JwtResponseAlgorithm>,
    /// Key ID (kid) for JWT header - useful for key rotation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// JWT issuer (iss claim)
    #[serde(default)]
    pub issuer: String,
    /// JWT audience (aud claim)
    #[serde(default)]
    pub audience: String,
    /// JWT subject (sub claim)
    #[serde(default)]
    pub subject: String,
    /// Token expiration time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
    /// Additional custom claims to include in the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, String>>,
    /// Token endpoint URL for exchanging JWT for access token
    #[serde(default)]
    pub token_url: String,
    /// OAuth2 scopes to request when exchanging JWT for access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Token field to extract from the token endpoint response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_response_field: Option<OAuth2JwtResponseTokenResponseField>,
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

impl OAuth2JwtResponse {
    pub fn builder() -> OAuth2JwtResponseBuilder {
        <OAuth2JwtResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuth2JwtResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    algorithm: Option<OAuth2JwtResponseAlgorithm>,
    key_id: Option<String>,
    issuer: Option<String>,
    audience: Option<String>,
    subject: Option<String>,
    expiration_seconds: Option<i64>,
    extra_params: Option<HashMap<String, String>>,
    token_url: Option<String>,
    scopes: Option<Vec<String>>,
    token_response_field: Option<OAuth2JwtResponseTokenResponseField>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl OAuth2JwtResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn algorithm(mut self, value: OAuth2JwtResponseAlgorithm) -> Self {
        self.algorithm = Some(value);
        self
    }

    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.key_id = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn audience(mut self, value: impl Into<String>) -> Self {
        self.audience = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn expiration_seconds(mut self, value: i64) -> Self {
        self.expiration_seconds = Some(value);
        self
    }

    pub fn extra_params(mut self, value: HashMap<String, String>) -> Self {
        self.extra_params = Some(value);
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

    pub fn token_response_field(mut self, value: OAuth2JwtResponseTokenResponseField) -> Self {
        self.token_response_field = Some(value);
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

    /// Consumes the builder and constructs a [`OAuth2JwtResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OAuth2JwtResponseBuilder::name)
    /// - [`provider`](OAuth2JwtResponseBuilder::provider)
    /// - [`issuer`](OAuth2JwtResponseBuilder::issuer)
    /// - [`audience`](OAuth2JwtResponseBuilder::audience)
    /// - [`subject`](OAuth2JwtResponseBuilder::subject)
    /// - [`token_url`](OAuth2JwtResponseBuilder::token_url)
    /// - [`id`](OAuth2JwtResponseBuilder::id)
    pub fn build(self) -> Result<OAuth2JwtResponse, BuildError> {
        Ok(OAuth2JwtResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            algorithm: self.algorithm,
            key_id: self.key_id,
            issuer: self.issuer.ok_or_else(|| BuildError::missing_field("issuer"))?,
            audience: self.audience.ok_or_else(|| BuildError::missing_field("audience"))?,
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
            expiration_seconds: self.expiration_seconds,
            extra_params: self.extra_params,
            token_url: self.token_url.ok_or_else(|| BuildError::missing_field("token_url"))?,
            scopes: self.scopes,
            token_response_field: self.token_response_field,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
