pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingSessionStarted {
    pub message_type: String,
    #[serde(default)]
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_id: Option<String>,
}

impl SubtitlingSessionStarted {
    pub fn builder() -> SubtitlingSessionStartedBuilder {
        <SubtitlingSessionStartedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingSessionStartedBuilder {
    message_type: Option<String>,
    session_id: Option<String>,
    client_session_id: Option<String>,
}

impl SubtitlingSessionStartedBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn client_session_id(mut self, value: impl Into<String>) -> Self {
        self.client_session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingSessionStarted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](SubtitlingSessionStartedBuilder::message_type)
    /// - [`session_id`](SubtitlingSessionStartedBuilder::session_id)
    pub fn build(self) -> Result<SubtitlingSessionStarted, BuildError> {
        Ok(SubtitlingSessionStarted {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            client_session_id: self.client_session_id,
        })
    }
}
