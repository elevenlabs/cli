pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeAgentConversationTicketsRequestModel {
    /// Open tickets of the same agent to fold into this one. They are kept with status 'merged', pointing at this ticket.
    #[serde(default)]
    pub source_ticket_ids: Vec<String>,
}

impl MergeAgentConversationTicketsRequestModel {
    pub fn builder() -> MergeAgentConversationTicketsRequestModelBuilder {
        <MergeAgentConversationTicketsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeAgentConversationTicketsRequestModelBuilder {
    source_ticket_ids: Option<Vec<String>>,
}

impl MergeAgentConversationTicketsRequestModelBuilder {
    pub fn source_ticket_ids(mut self, value: Vec<String>) -> Self {
        self.source_ticket_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeAgentConversationTicketsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_ticket_ids`](MergeAgentConversationTicketsRequestModelBuilder::source_ticket_ids)
    pub fn build(self) -> Result<MergeAgentConversationTicketsRequestModel, BuildError> {
        Ok(MergeAgentConversationTicketsRequestModel {
            source_ticket_ids: self.source_ticket_ids.ok_or_else(|| BuildError::missing_field("source_ticket_ids"))?,
        })
    }
}

