pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateManualTicketRequestModel {
    /// What the ticket is about, e.g. a follow-up task for the agent. This is shown as the ticket title.
    #[serde(default)]
    pub qa_comment: String,
}

impl CreateManualTicketRequestModel {
    pub fn builder() -> CreateManualTicketRequestModelBuilder {
        <CreateManualTicketRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManualTicketRequestModelBuilder {
    qa_comment: Option<String>,
}

impl CreateManualTicketRequestModelBuilder {
    pub fn qa_comment(mut self, value: impl Into<String>) -> Self {
        self.qa_comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateManualTicketRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`qa_comment`](CreateManualTicketRequestModelBuilder::qa_comment)
    pub fn build(self) -> Result<CreateManualTicketRequestModel, BuildError> {
        Ok(CreateManualTicketRequestModel {
            qa_comment: self.qa_comment.ok_or_else(|| BuildError::missing_field("qa_comment"))?,
        })
    }
}

