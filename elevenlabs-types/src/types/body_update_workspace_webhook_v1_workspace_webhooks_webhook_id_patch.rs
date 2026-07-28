pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatch {
    /// Whether to disable or enable the webhook
    #[serde(default)]
    pub is_disabled: bool,
    /// The display name of the webhook (used for display purposes only).
    #[serde(default)]
    pub name: String,
    /// Whether to enable automatic retries for transient failures (5xx, 429, timeout)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_enabled: Option<bool>,
    /// A list of request headers to include with the webhook delivery (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, Option<String>>>,
    /// The complete set of workspace-level events this webhook should be subscribed to. The webhook is added to the events in the list and removed from any not in the list. Omit to leave the current event subscriptions unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WorkspaceWebhookEventType>>,
}

impl BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatch {
    pub fn builder() -> BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatchBuilder {
        <BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatchBuilder {
    is_disabled: Option<bool>,
    name: Option<String>,
    retry_enabled: Option<bool>,
    request_headers: Option<HashMap<String, Option<String>>>,
    events: Option<Vec<WorkspaceWebhookEventType>>,
}

impl BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatchBuilder {
    pub fn is_disabled(mut self, value: bool) -> Self {
        self.is_disabled = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn retry_enabled(mut self, value: bool) -> Self {
        self.retry_enabled = Some(value);
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<WorkspaceWebhookEventType>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_disabled`](BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatchBuilder::is_disabled)
    /// - [`name`](BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatchBuilder::name)
    pub fn build(self) -> Result<BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatch, BuildError> {
        Ok(BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatch {
            is_disabled: self.is_disabled.ok_or_else(|| BuildError::missing_field("is_disabled"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            retry_enabled: self.retry_enabled,
            request_headers: self.request_headers,
            events: self.events,
        })
    }
}

