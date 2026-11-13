pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationProgress {
    /// Calls due to be placed that haven't started.
    #[serde(default)]
    pub scheduled: i64,
    /// Scheduled calls by call type. Missing keys mean none.
    #[serde(default)]
    pub scheduled_by_type: HashMap<String, i64>,
    /// WhatsApp calls waiting on the contact to grant call permission.
    #[serde(default)]
    pub awaiting_permission: i64,
    /// Calls under way.
    #[serde(default)]
    pub in_progress: i64,
    /// Ended calls whose analysis the campaign is waiting on.
    #[serde(default)]
    pub awaiting_analysis: i64,
}

impl ConversationProgress {
    pub fn builder() -> ConversationProgressBuilder {
        <ConversationProgressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationProgressBuilder {
    scheduled: Option<i64>,
    scheduled_by_type: Option<HashMap<String, i64>>,
    awaiting_permission: Option<i64>,
    in_progress: Option<i64>,
    awaiting_analysis: Option<i64>,
}

impl ConversationProgressBuilder {
    pub fn scheduled(mut self, value: i64) -> Self {
        self.scheduled = Some(value);
        self
    }

    pub fn scheduled_by_type(mut self, value: HashMap<String, i64>) -> Self {
        self.scheduled_by_type = Some(value);
        self
    }

    pub fn awaiting_permission(mut self, value: i64) -> Self {
        self.awaiting_permission = Some(value);
        self
    }

    pub fn in_progress(mut self, value: i64) -> Self {
        self.in_progress = Some(value);
        self
    }

    pub fn awaiting_analysis(mut self, value: i64) -> Self {
        self.awaiting_analysis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationProgress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheduled`](ConversationProgressBuilder::scheduled)
    /// - [`scheduled_by_type`](ConversationProgressBuilder::scheduled_by_type)
    /// - [`awaiting_permission`](ConversationProgressBuilder::awaiting_permission)
    /// - [`in_progress`](ConversationProgressBuilder::in_progress)
    /// - [`awaiting_analysis`](ConversationProgressBuilder::awaiting_analysis)
    pub fn build(self) -> Result<ConversationProgress, BuildError> {
        Ok(ConversationProgress {
            scheduled: self.scheduled.ok_or_else(|| BuildError::missing_field("scheduled"))?,
            scheduled_by_type: self.scheduled_by_type.ok_or_else(|| BuildError::missing_field("scheduled_by_type"))?,
            awaiting_permission: self.awaiting_permission.ok_or_else(|| BuildError::missing_field("awaiting_permission"))?,
            in_progress: self.in_progress.ok_or_else(|| BuildError::missing_field("in_progress"))?,
            awaiting_analysis: self.awaiting_analysis.ok_or_else(|| BuildError::missing_field("awaiting_analysis"))?,
        })
    }
}
