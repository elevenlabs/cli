pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for create
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateQueryRequest {
    /// Closes the realtime transcription session opened with this token after this many seconds. Only for realtime_scribe tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration_seconds: Option<i64>,
}

impl CreateQueryRequest {
    pub fn builder() -> CreateQueryRequestBuilder {
        <CreateQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQueryRequestBuilder {
    max_session_duration_seconds: Option<i64>,
}

impl CreateQueryRequestBuilder {
    pub fn max_session_duration_seconds(mut self, value: i64) -> Self {
        self.max_session_duration_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateQueryRequest`].
    pub fn build(self) -> Result<CreateQueryRequest, BuildError> {
        Ok(CreateQueryRequest {
            max_session_duration_seconds: self.max_session_duration_seconds,
        })
    }
}

