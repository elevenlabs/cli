pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for basic auth
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BasicAuthResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub username: String,
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

impl BasicAuthResponse {
    pub fn builder() -> BasicAuthResponseBuilder {
        <BasicAuthResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicAuthResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    username: Option<String>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl BasicAuthResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
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

    /// Consumes the builder and constructs a [`BasicAuthResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BasicAuthResponseBuilder::name)
    /// - [`provider`](BasicAuthResponseBuilder::provider)
    /// - [`username`](BasicAuthResponseBuilder::username)
    /// - [`id`](BasicAuthResponseBuilder::id)
    pub fn build(self) -> Result<BasicAuthResponse, BuildError> {
        Ok(BasicAuthResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
