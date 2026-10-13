pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketStatusChangeResponseModel {
    pub status: AgentConversationTicketStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_by_user_id: Option<String>,
    #[serde(default)]
    pub changed_at_unix_secs: i64,
}

impl TicketStatusChangeResponseModel {
    pub fn builder() -> TicketStatusChangeResponseModelBuilder {
        <TicketStatusChangeResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketStatusChangeResponseModelBuilder {
    status: Option<AgentConversationTicketStatus>,
    changed_by_user_id: Option<String>,
    changed_at_unix_secs: Option<i64>,
}

impl TicketStatusChangeResponseModelBuilder {
    pub fn status(mut self, value: AgentConversationTicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn changed_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.changed_by_user_id = Some(value.into());
        self
    }

    pub fn changed_at_unix_secs(mut self, value: i64) -> Self {
        self.changed_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketStatusChangeResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](TicketStatusChangeResponseModelBuilder::status)
    /// - [`changed_at_unix_secs`](TicketStatusChangeResponseModelBuilder::changed_at_unix_secs)
    pub fn build(self) -> Result<TicketStatusChangeResponseModel, BuildError> {
        Ok(TicketStatusChangeResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            changed_by_user_id: self.changed_by_user_id,
            changed_at_unix_secs: self.changed_at_unix_secs.ok_or_else(|| BuildError::missing_field("changed_at_unix_secs"))?,
        })
    }
}
