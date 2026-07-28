pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PendingCancellationResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The timestamp of the cancellation.
    #[serde(default)]
    pub timestamp_seconds: i64,
}

impl PendingCancellationResponseModel {
    pub fn builder() -> PendingCancellationResponseModelBuilder {
        <PendingCancellationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingCancellationResponseModelBuilder {
    kind: Option<String>,
    timestamp_seconds: Option<i64>,
}

impl PendingCancellationResponseModelBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn timestamp_seconds(mut self, value: i64) -> Self {
        self.timestamp_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingCancellationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp_seconds`](PendingCancellationResponseModelBuilder::timestamp_seconds)
    pub fn build(self) -> Result<PendingCancellationResponseModel, BuildError> {
        Ok(PendingCancellationResponseModel {
            kind: self.kind,
            timestamp_seconds: self.timestamp_seconds.ok_or_else(|| BuildError::missing_field("timestamp_seconds"))?,
        })
    }
}
