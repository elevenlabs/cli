pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceBatchCallsResponse {
    #[serde(default)]
    pub batch_calls: Vec<BatchCallResponse>,
    /// The next document, used to paginate through the batch calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_doc: Option<String>,
    /// Whether there are more batch calls to paginate through
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl WorkspaceBatchCallsResponse {
    pub fn builder() -> WorkspaceBatchCallsResponseBuilder {
        <WorkspaceBatchCallsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceBatchCallsResponseBuilder {
    batch_calls: Option<Vec<BatchCallResponse>>,
    next_doc: Option<String>,
    has_more: Option<bool>,
}

impl WorkspaceBatchCallsResponseBuilder {
    pub fn batch_calls(mut self, value: Vec<BatchCallResponse>) -> Self {
        self.batch_calls = Some(value);
        self
    }

    pub fn next_doc(mut self, value: impl Into<String>) -> Self {
        self.next_doc = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceBatchCallsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_calls`](WorkspaceBatchCallsResponseBuilder::batch_calls)
    pub fn build(self) -> Result<WorkspaceBatchCallsResponse, BuildError> {
        Ok(WorkspaceBatchCallsResponse {
            batch_calls: self.batch_calls.ok_or_else(|| BuildError::missing_field("batch_calls"))?,
            next_doc: self.next_doc,
            has_more: self.has_more,
        })
    }
}
