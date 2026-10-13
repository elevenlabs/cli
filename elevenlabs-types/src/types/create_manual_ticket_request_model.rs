pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateManualTicketRequestModel {
    /// One-line headline shown in the triage list. Defaults to one derived from qa_comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What the ticket is about, e.g. a follow-up task for the agent.
    #[serde(default)]
    pub qa_comment: String,
    /// How urgently the ticket needs attention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AgentConversationTicketPriority>,
}

impl CreateManualTicketRequestModel {
    pub fn builder() -> CreateManualTicketRequestModelBuilder {
        <CreateManualTicketRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManualTicketRequestModelBuilder {
    title: Option<String>,
    qa_comment: Option<String>,
    priority: Option<AgentConversationTicketPriority>,
}

impl CreateManualTicketRequestModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn qa_comment(mut self, value: impl Into<String>) -> Self {
        self.qa_comment = Some(value.into());
        self
    }

    pub fn priority(mut self, value: AgentConversationTicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateManualTicketRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`qa_comment`](CreateManualTicketRequestModelBuilder::qa_comment)
    pub fn build(self) -> Result<CreateManualTicketRequestModel, BuildError> {
        Ok(CreateManualTicketRequestModel {
            title: self.title,
            qa_comment: self.qa_comment.ok_or_else(|| BuildError::missing_field("qa_comment"))?,
            priority: self.priority,
        })
    }
}

