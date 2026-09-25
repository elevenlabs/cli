pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingPagerDutyNotifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    /// ID of the workspace integration connection to deliver alert lifecycle notifications to. The connection's integration must have the monitoring capability and match ``integration_type``.
    #[serde(default)]
    pub connection_id: String,
}

impl AlertingPagerDutyNotifier {
    pub fn builder() -> AlertingPagerDutyNotifierBuilder {
        <AlertingPagerDutyNotifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingPagerDutyNotifierBuilder {
    r#type: Option<String>,
    integration_type: Option<String>,
    connection_id: Option<String>,
}

impl AlertingPagerDutyNotifierBuilder {
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

    /// Consumes the builder and constructs a [`AlertingPagerDutyNotifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](AlertingPagerDutyNotifierBuilder::connection_id)
    pub fn build(self) -> Result<AlertingPagerDutyNotifier, BuildError> {
        Ok(AlertingPagerDutyNotifier {
            r#type: self.r#type,
            integration_type: self.integration_type,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
        })
    }
}
