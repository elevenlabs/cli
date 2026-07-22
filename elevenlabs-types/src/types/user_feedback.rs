pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserFeedback {
    pub score: UserFeedbackScore,
    #[serde(default)]
    pub time_in_call_secs: i64,
}

impl UserFeedback {
    pub fn builder() -> UserFeedbackBuilder {
        <UserFeedbackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserFeedbackBuilder {
    score: Option<UserFeedbackScore>,
    time_in_call_secs: Option<i64>,
}

impl UserFeedbackBuilder {
    pub fn score(mut self, value: UserFeedbackScore) -> Self {
        self.score = Some(value);
        self
    }

    pub fn time_in_call_secs(mut self, value: i64) -> Self {
        self.time_in_call_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserFeedback`].
    /// This method will fail if any of the following fields are not set:
    /// - [`score`](UserFeedbackBuilder::score)
    /// - [`time_in_call_secs`](UserFeedbackBuilder::time_in_call_secs)
    pub fn build(self) -> Result<UserFeedback, BuildError> {
        Ok(UserFeedback {
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
            time_in_call_secs: self.time_in_call_secs.ok_or_else(|| BuildError::missing_field("time_in_call_secs"))?,
        })
    }
}
