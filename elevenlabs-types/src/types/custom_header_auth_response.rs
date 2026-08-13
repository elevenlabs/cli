pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for Custom Header Auth auth connections
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomHeaderAuthResponse {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider: String,
    /// The name of the header to use for authentication (e.g., 'x-api-key')
    #[serde(default)]
    pub header_name: String,
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

impl CustomHeaderAuthResponse {
    pub fn builder() -> CustomHeaderAuthResponseBuilder {
        <CustomHeaderAuthResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomHeaderAuthResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    header_name: Option<String>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl CustomHeaderAuthResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn header_name(mut self, value: impl Into<String>) -> Self {
        self.header_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`CustomHeaderAuthResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CustomHeaderAuthResponseBuilder::name)
    /// - [`provider`](CustomHeaderAuthResponseBuilder::provider)
    /// - [`header_name`](CustomHeaderAuthResponseBuilder::header_name)
    /// - [`id`](CustomHeaderAuthResponseBuilder::id)
    pub fn build(self) -> Result<CustomHeaderAuthResponse, BuildError> {
        Ok(CustomHeaderAuthResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            header_name: self.header_name.ok_or_else(|| BuildError::missing_field("header_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
