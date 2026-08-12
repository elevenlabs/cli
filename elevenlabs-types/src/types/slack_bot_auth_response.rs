pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for the internal Slack BYO bot auth connection.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SlackBotAuthResponse {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
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

impl SlackBotAuthResponse {
    pub fn builder() -> SlackBotAuthResponseBuilder {
        <SlackBotAuthResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackBotAuthResponseBuilder {
    name: Option<String>,
    provider: Option<String>,
    id: Option<String>,
    used_by: Option<AuthConnectionDependencies>,
    status: Option<AuthConnectionStatus>,
    status_detail: Option<String>,
    status_updated_at: Option<String>,
}

impl SlackBotAuthResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
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

    /// Consumes the builder and constructs a [`SlackBotAuthResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SlackBotAuthResponseBuilder::name)
    /// - [`id`](SlackBotAuthResponseBuilder::id)
    pub fn build(self) -> Result<SlackBotAuthResponse, BuildError> {
        Ok(SlackBotAuthResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            provider: self.provider,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            used_by: self.used_by,
            status: self.status,
            status_detail: self.status_detail,
            status_updated_at: self.status_updated_at,
        })
    }
}
