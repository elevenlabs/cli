pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryFeedbackCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConversationFeedbackType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_score: Option<UserFeedbackScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dislikes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ConversationHistoryFeedbackCommonModel {
    pub fn builder() -> ConversationHistoryFeedbackCommonModelBuilder {
        <ConversationHistoryFeedbackCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryFeedbackCommonModelBuilder {
    r#type: Option<ConversationFeedbackType>,
    overall_score: Option<UserFeedbackScore>,
    likes: Option<i64>,
    dislikes: Option<i64>,
    rating: Option<i64>,
    comment: Option<String>,
}

impl ConversationHistoryFeedbackCommonModelBuilder {
    pub fn r#type(mut self, value: ConversationFeedbackType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn overall_score(mut self, value: UserFeedbackScore) -> Self {
        self.overall_score = Some(value);
        self
    }

    pub fn likes(mut self, value: i64) -> Self {
        self.likes = Some(value);
        self
    }

    pub fn dislikes(mut self, value: i64) -> Self {
        self.dislikes = Some(value);
        self
    }

    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryFeedbackCommonModel`].
    pub fn build(self) -> Result<ConversationHistoryFeedbackCommonModel, BuildError> {
        Ok(ConversationHistoryFeedbackCommonModel {
            r#type: self.r#type,
            overall_score: self.overall_score,
            likes: self.likes,
            dislikes: self.dislikes,
            rating: self.rating,
            comment: self.comment,
        })
    }
}
