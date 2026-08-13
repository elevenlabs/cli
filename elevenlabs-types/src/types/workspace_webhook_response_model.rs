pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceWebhookResponseModel {
    /// The display name for this webhook.
    #[serde(default)]
    pub name: String,
    /// The unique ID for this webhook.
    #[serde(default)]
    pub webhook_id: String,
    /// The HTTPS callback URL that is called when this webhook is triggered in the platform.
    #[serde(default)]
    pub webhook_url: String,
    /// Whether the webhook has been manually disabled by a user.
    #[serde(default)]
    pub is_disabled: bool,
    /// Whether the webhook has been automatically disabled due to repeated consecutive failures over a long period of time.
    #[serde(default)]
    pub is_auto_disabled: bool,
    /// Original creation time of the webhook.
    #[serde(default)]
    pub created_at_unix: i64,
    /// The authentication mode used to secure the webhook.
    pub auth_type: WebhookAuthMethodType,
    /// The list of products that are currently configured to trigger this webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<WorkspaceWebhookUsageResponseModel>>,
    /// The workspace-level events this webhook is currently subscribed to. Only populated when usages are requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WorkspaceWebhookEventType>>,
    /// The most recent error code returned from the callback URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_failure_error_code: Option<i64>,
    /// The most recent time the webhook failed, failures are any non-200 codes returned by the callback URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_failure_timestamp: Option<i64>,
}

impl WorkspaceWebhookResponseModel {
    pub fn builder() -> WorkspaceWebhookResponseModelBuilder {
        <WorkspaceWebhookResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceWebhookResponseModelBuilder {
    name: Option<String>,
    webhook_id: Option<String>,
    webhook_url: Option<String>,
    is_disabled: Option<bool>,
    is_auto_disabled: Option<bool>,
    created_at_unix: Option<i64>,
    auth_type: Option<WebhookAuthMethodType>,
    usage: Option<Vec<WorkspaceWebhookUsageResponseModel>>,
    events: Option<Vec<WorkspaceWebhookEventType>>,
    most_recent_failure_error_code: Option<i64>,
    most_recent_failure_timestamp: Option<i64>,
}

impl WorkspaceWebhookResponseModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn is_disabled(mut self, value: bool) -> Self {
        self.is_disabled = Some(value);
        self
    }

    pub fn is_auto_disabled(mut self, value: bool) -> Self {
        self.is_auto_disabled = Some(value);
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn auth_type(mut self, value: WebhookAuthMethodType) -> Self {
        self.auth_type = Some(value);
        self
    }

    pub fn usage(mut self, value: Vec<WorkspaceWebhookUsageResponseModel>) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<WorkspaceWebhookEventType>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn most_recent_failure_error_code(mut self, value: i64) -> Self {
        self.most_recent_failure_error_code = Some(value);
        self
    }

    pub fn most_recent_failure_timestamp(mut self, value: i64) -> Self {
        self.most_recent_failure_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceWebhookResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkspaceWebhookResponseModelBuilder::name)
    /// - [`webhook_id`](WorkspaceWebhookResponseModelBuilder::webhook_id)
    /// - [`webhook_url`](WorkspaceWebhookResponseModelBuilder::webhook_url)
    /// - [`is_disabled`](WorkspaceWebhookResponseModelBuilder::is_disabled)
    /// - [`is_auto_disabled`](WorkspaceWebhookResponseModelBuilder::is_auto_disabled)
    /// - [`created_at_unix`](WorkspaceWebhookResponseModelBuilder::created_at_unix)
    /// - [`auth_type`](WorkspaceWebhookResponseModelBuilder::auth_type)
    pub fn build(self) -> Result<WorkspaceWebhookResponseModel, BuildError> {
        Ok(WorkspaceWebhookResponseModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            webhook_id: self.webhook_id.ok_or_else(|| BuildError::missing_field("webhook_id"))?,
            webhook_url: self.webhook_url.ok_or_else(|| BuildError::missing_field("webhook_url"))?,
            is_disabled: self.is_disabled.ok_or_else(|| BuildError::missing_field("is_disabled"))?,
            is_auto_disabled: self.is_auto_disabled.ok_or_else(|| BuildError::missing_field("is_auto_disabled"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            auth_type: self.auth_type.ok_or_else(|| BuildError::missing_field("auth_type"))?,
            usage: self.usage,
            events: self.events,
            most_recent_failure_error_code: self.most_recent_failure_error_code,
            most_recent_failure_timestamp: self.most_recent_failure_timestamp,
        })
    }
}
