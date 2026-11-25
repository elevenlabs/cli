pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingStatus {
    pub message_type: String,
    #[serde(default)]
    pub status: String,
}

impl SubtitlingStatus {
    pub fn builder() -> SubtitlingStatusBuilder {
        <SubtitlingStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingStatusBuilder {
    message_type: Option<String>,
    status: Option<String>,
}

impl SubtitlingStatusBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](SubtitlingStatusBuilder::message_type)
    /// - [`status`](SubtitlingStatusBuilder::status)
    pub fn build(self) -> Result<SubtitlingStatus, BuildError> {
        Ok(SubtitlingStatus {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
