pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPost {
    /// The event type to send a sample of.
    pub webhook_type: WebhookTypes,
}

impl BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPost {
    pub fn builder() -> BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPostBuilder {
        <BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPostBuilder {
    webhook_type: Option<WebhookTypes>,
}

impl BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPostBuilder {
    pub fn webhook_type(mut self, value: WebhookTypes) -> Self {
        self.webhook_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_type`](BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPostBuilder::webhook_type)
    pub fn build(self) -> Result<BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPost, BuildError> {
        Ok(BodySendTestWebhookEventV1WorkspaceWebhooksWebhookIdTestPost {
            webhook_type: self.webhook_type.ok_or_else(|| BuildError::missing_field("webhook_type"))?,
        })
    }
}

