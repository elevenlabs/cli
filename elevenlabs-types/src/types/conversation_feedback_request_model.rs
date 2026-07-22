pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationFeedbackRequestModel {
    /// Either 'like' or 'dislike' to indicate the feedback for the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<UserFeedbackScore>,
}

impl ConversationFeedbackRequestModel {
    pub fn builder() -> ConversationFeedbackRequestModelBuilder {
        <ConversationFeedbackRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationFeedbackRequestModelBuilder {
    feedback: Option<UserFeedbackScore>,
}

impl ConversationFeedbackRequestModelBuilder {
    pub fn feedback(mut self, value: UserFeedbackScore) -> Self {
        self.feedback = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationFeedbackRequestModel`].
    pub fn build(self) -> Result<ConversationFeedbackRequestModel, BuildError> {
        Ok(ConversationFeedbackRequestModel {
            feedback: self.feedback,
        })
    }
}

