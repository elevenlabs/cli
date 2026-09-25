pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingSlackNotifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// ID of the workspace integration connection to deliver alert lifecycle notifications to. The connection's integration must have the monitoring capability and match ``integration_type``.
    #[serde(default)]
    pub connection_id: String,
    /// ID of the Slack channel to post alert notifications to, e.g. ``C0123456789``. The Slack app must be a member of the channel and have the ``chat:write`` scope, or ``chat:write.public`` for public channels it has not joined.
    #[serde(default)]
    pub channel_id: String,
}

impl AlertingSlackNotifier {
    pub fn builder() -> AlertingSlackNotifierBuilder {
        <AlertingSlackNotifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingSlackNotifierBuilder {
    r#type: Option<String>,
    integration_type: Option<String>,
    connection_id: Option<String>,
    channel_id: Option<String>,
}

impl AlertingSlackNotifierBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn integration_type(mut self, value: impl Into<String>) -> Self {
        self.integration_type = Some(value.into());
        self
    }

    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingSlackNotifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](AlertingSlackNotifierBuilder::connection_id)
    /// - [`channel_id`](AlertingSlackNotifierBuilder::channel_id)
    pub fn build(self) -> Result<AlertingSlackNotifier, BuildError> {
        Ok(AlertingSlackNotifier {
            r#type: self.r#type,
            integration_type: self.integration_type,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            channel_id: self.channel_id.ok_or_else(|| BuildError::missing_field("channel_id"))?,
        })
    }
}
