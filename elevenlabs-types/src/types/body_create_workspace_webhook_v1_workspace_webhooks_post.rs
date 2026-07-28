pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost {
    /// Webhook settings object containing auth_type and corresponding configuration
    pub settings: WebhookHmacSettings,
}

impl BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost {
    pub fn builder() -> BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPostBuilder {
        <BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPostBuilder {
    settings: Option<WebhookHmacSettings>,
}

impl BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPostBuilder {
    pub fn settings(mut self, value: WebhookHmacSettings) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`settings`](BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPostBuilder::settings)
    pub fn build(self) -> Result<BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost, BuildError> {
        Ok(BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost {
            settings: self.settings.ok_or_else(|| BuildError::missing_field("settings"))?,
        })
    }
}

