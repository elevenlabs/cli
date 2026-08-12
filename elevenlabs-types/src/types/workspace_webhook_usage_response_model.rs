pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceWebhookUsageResponseModel {
    pub usage_type: WebhookUsageType,
}

impl WorkspaceWebhookUsageResponseModel {
    pub fn builder() -> WorkspaceWebhookUsageResponseModelBuilder {
        <WorkspaceWebhookUsageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceWebhookUsageResponseModelBuilder {
    usage_type: Option<WebhookUsageType>,
}

impl WorkspaceWebhookUsageResponseModelBuilder {
    pub fn usage_type(mut self, value: WebhookUsageType) -> Self {
        self.usage_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceWebhookUsageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`usage_type`](WorkspaceWebhookUsageResponseModelBuilder::usage_type)
    pub fn build(self) -> Result<WorkspaceWebhookUsageResponseModel, BuildError> {
        Ok(WorkspaceWebhookUsageResponseModel {
            usage_type: self.usage_type.ok_or_else(|| BuildError::missing_field("usage_type"))?,
        })
    }
}
