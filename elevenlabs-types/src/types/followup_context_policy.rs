pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which `campaign__` dynamic variables follow-up calls get.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FollowupContextPolicy {
    /// Pass the call type as `campaign__call_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_type: Option<bool>,
    /// Pass the last conversation's ID as `campaign__previous_conversation_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_conversation_id: Option<bool>,
    /// Pass the last conversation's summary as `campaign__previous_summary`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_summary: Option<bool>,
    /// Pass the last conversation's transcript as `campaign__previous_transcript`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_transcript: Option<bool>,
}

impl FollowupContextPolicy {
    pub fn builder() -> FollowupContextPolicyBuilder {
        <FollowupContextPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FollowupContextPolicyBuilder {
    call_type: Option<bool>,
    previous_conversation_id: Option<bool>,
    previous_summary: Option<bool>,
    previous_transcript: Option<bool>,
}

impl FollowupContextPolicyBuilder {
    pub fn call_type(mut self, value: bool) -> Self {
        self.call_type = Some(value);
        self
    }

    pub fn previous_conversation_id(mut self, value: bool) -> Self {
        self.previous_conversation_id = Some(value);
        self
    }

    pub fn previous_summary(mut self, value: bool) -> Self {
        self.previous_summary = Some(value);
        self
    }

    pub fn previous_transcript(mut self, value: bool) -> Self {
        self.previous_transcript = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FollowupContextPolicy`].
    pub fn build(self) -> Result<FollowupContextPolicy, BuildError> {
        Ok(FollowupContextPolicy {
            call_type: self.call_type,
            previous_conversation_id: self.previous_conversation_id,
            previous_summary: self.previous_summary,
            previous_transcript: self.previous_transcript,
        })
    }
}
