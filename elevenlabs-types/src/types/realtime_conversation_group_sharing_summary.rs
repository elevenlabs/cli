pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How widely a group is shared, besides its owner and workspace admins (who always have access).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RealtimeConversationGroupSharingSummary {
    /// Distinct people besides the owner with access, whether shared directly or through a workspace group they belong to.
    #[serde(default)]
    pub people_count: i64,
    /// Whether every workspace member has access to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_with_everyone: Option<bool>,
    /// The preset role every workspace member has, when shared with everyone. None for a workspace custom role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub everyone_role: Option<RealtimeConversationGroupSharingSummaryEveryoneRole>,
}

impl RealtimeConversationGroupSharingSummary {
    pub fn builder() -> RealtimeConversationGroupSharingSummaryBuilder {
        <RealtimeConversationGroupSharingSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RealtimeConversationGroupSharingSummaryBuilder {
    people_count: Option<i64>,
    shared_with_everyone: Option<bool>,
    everyone_role: Option<RealtimeConversationGroupSharingSummaryEveryoneRole>,
}

impl RealtimeConversationGroupSharingSummaryBuilder {
    pub fn people_count(mut self, value: i64) -> Self {
        self.people_count = Some(value);
        self
    }

    pub fn shared_with_everyone(mut self, value: bool) -> Self {
        self.shared_with_everyone = Some(value);
        self
    }

    pub fn everyone_role(mut self, value: RealtimeConversationGroupSharingSummaryEveryoneRole) -> Self {
        self.everyone_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RealtimeConversationGroupSharingSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`people_count`](RealtimeConversationGroupSharingSummaryBuilder::people_count)
    pub fn build(self) -> Result<RealtimeConversationGroupSharingSummary, BuildError> {
        Ok(RealtimeConversationGroupSharingSummary {
            people_count: self.people_count.ok_or_else(|| BuildError::missing_field("people_count"))?,
            shared_with_everyone: self.shared_with_everyone,
            everyone_role: self.everyone_role,
        })
    }
}
