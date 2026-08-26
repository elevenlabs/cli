pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddTicketCommentRequestModel {
    /// A comment discussing how to resolve the ticket.
    #[serde(default)]
    pub comment: String,
}

impl AddTicketCommentRequestModel {
    pub fn builder() -> AddTicketCommentRequestModelBuilder {
        <AddTicketCommentRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddTicketCommentRequestModelBuilder {
    comment: Option<String>,
}

impl AddTicketCommentRequestModelBuilder {
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddTicketCommentRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`comment`](AddTicketCommentRequestModelBuilder::comment)
    pub fn build(self) -> Result<AddTicketCommentRequestModel, BuildError> {
        Ok(AddTicketCommentRequestModel {
            comment: self.comment.ok_or_else(|| BuildError::missing_field("comment"))?,
        })
    }
}

