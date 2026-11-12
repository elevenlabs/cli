pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingStatus {
    pub message_type: String,
    pub status: DubbingStatusStatus,
}

impl DubbingStatus {
    pub fn builder() -> DubbingStatusBuilder {
        <DubbingStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingStatusBuilder {
    message_type: Option<String>,
    status: Option<DubbingStatusStatus>,
}

impl DubbingStatusBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: DubbingStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](DubbingStatusBuilder::message_type)
    /// - [`status`](DubbingStatusBuilder::status)
    pub fn build(self) -> Result<DubbingStatus, BuildError> {
        Ok(DubbingStatus {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
