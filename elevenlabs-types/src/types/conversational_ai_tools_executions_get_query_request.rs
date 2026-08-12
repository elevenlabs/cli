pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationalAiToolsExecutionsGetQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter by error status. If not provided, returns all executions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    /// Filter by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter by agent branch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Filter executions from this Unix timestamp (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start_time: Option<f64>,
    /// Filter executions until this Unix timestamp (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end_time: Option<f64>,
}

impl ConversationalAiToolsExecutionsGetQueryRequest {
    pub fn builder() -> ConversationalAiToolsExecutionsGetQueryRequestBuilder {
        <ConversationalAiToolsExecutionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiToolsExecutionsGetQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    is_error: Option<bool>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    start_time: Option<f64>,
    end_time: Option<f64>,
}

impl ConversationalAiToolsExecutionsGetQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn start_time(mut self, value: f64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: f64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiToolsExecutionsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiToolsExecutionsGetQueryRequest, BuildError> {
        Ok(ConversationalAiToolsExecutionsGetQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            is_error: self.is_error,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

