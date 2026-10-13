pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TicketPriorityChangeResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AgentConversationTicketPriority>,
    #[serde(default)]
    pub changed_by_user_id: String,
    #[serde(default)]
    pub changed_at_unix_secs: i64,
}

impl TicketPriorityChangeResponseModel {
    pub fn builder() -> TicketPriorityChangeResponseModelBuilder {
        <TicketPriorityChangeResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketPriorityChangeResponseModelBuilder {
    priority: Option<AgentConversationTicketPriority>,
    changed_by_user_id: Option<String>,
    changed_at_unix_secs: Option<i64>,
}

impl TicketPriorityChangeResponseModelBuilder {
    pub fn priority(mut self, value: AgentConversationTicketPriority) -> Self {
        self.priority = Some(value);
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

    /// Consumes the builder and constructs a [`TicketPriorityChangeResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`changed_by_user_id`](TicketPriorityChangeResponseModelBuilder::changed_by_user_id)
    /// - [`changed_at_unix_secs`](TicketPriorityChangeResponseModelBuilder::changed_at_unix_secs)
    pub fn build(self) -> Result<TicketPriorityChangeResponseModel, BuildError> {
        Ok(TicketPriorityChangeResponseModel {
            priority: self.priority,
            changed_by_user_id: self.changed_by_user_id.ok_or_else(|| BuildError::missing_field("changed_by_user_id"))?,
            changed_at_unix_secs: self.changed_at_unix_secs.ok_or_else(|| BuildError::missing_field("changed_at_unix_secs"))?,
        })
    }
}
