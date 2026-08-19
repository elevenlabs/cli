pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationSummaryMessageModel {
    pub role: ConversationSummaryMessageModelRole,
    #[serde(default)]
    pub message: String,
}

impl ConversationSummaryMessageModel {
    pub fn builder() -> ConversationSummaryMessageModelBuilder {
        <ConversationSummaryMessageModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationSummaryMessageModelBuilder {
    role: Option<ConversationSummaryMessageModelRole>,
    message: Option<String>,
}

impl ConversationSummaryMessageModelBuilder {
    pub fn role(mut self, value: ConversationSummaryMessageModelRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationSummaryMessageModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](ConversationSummaryMessageModelBuilder::role)
    /// - [`message`](ConversationSummaryMessageModelBuilder::message)
    pub fn build(self) -> Result<ConversationSummaryMessageModel, BuildError> {
        Ok(ConversationSummaryMessageModel {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
