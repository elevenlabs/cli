pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingPagerDutyNotifierResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(default)]
    pub connection_id: String,
}

impl AlertingPagerDutyNotifierResponse {
    pub fn builder() -> AlertingPagerDutyNotifierResponseBuilder {
        <AlertingPagerDutyNotifierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingPagerDutyNotifierResponseBuilder {
    r#type: Option<String>,
    integration_type: Option<String>,
    connection_id: Option<String>,
}

impl AlertingPagerDutyNotifierResponseBuilder {
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

    /// Consumes the builder and constructs a [`AlertingPagerDutyNotifierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](AlertingPagerDutyNotifierResponseBuilder::connection_id)
    pub fn build(self) -> Result<AlertingPagerDutyNotifierResponse, BuildError> {
        Ok(AlertingPagerDutyNotifierResponse {
            r#type: self.r#type,
            integration_type: self.integration_type,
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
        })
    }
}
