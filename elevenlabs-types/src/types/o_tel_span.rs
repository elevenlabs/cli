pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OTelSpan {
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(rename = "spanId")]
    #[serde(default)]
    pub span_id: String,
    #[serde(rename = "parentSpanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: i64,
    #[serde(rename = "startTimeUnixNano")]
    #[serde(default)]
    pub start_time_unix_nano: String,
    #[serde(rename = "endTimeUnixNano")]
    #[serde(default)]
    pub end_time_unix_nano: String,
    #[serde(default)]
    pub attributes: Vec<OTelAttribute>,
    #[serde(default)]
    pub status: OTelStatus,
}

impl OTelSpan {
    pub fn builder() -> OTelSpanBuilder {
        <OTelSpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelSpanBuilder {
    trace_id: Option<String>,
    span_id: Option<String>,
    parent_span_id: Option<String>,
    name: Option<String>,
    kind: Option<i64>,
    start_time_unix_nano: Option<String>,
    end_time_unix_nano: Option<String>,
    attributes: Option<Vec<OTelAttribute>>,
    status: Option<OTelStatus>,
}

impl OTelSpanBuilder {
    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn parent_span_id(mut self, value: impl Into<String>) -> Self {
        self.parent_span_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn kind(mut self, value: i64) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn start_time_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.start_time_unix_nano = Some(value.into());
        self
    }

    pub fn end_time_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.end_time_unix_nano = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: Vec<OTelAttribute>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn status(mut self, value: OTelStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OTelSpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_id`](OTelSpanBuilder::trace_id)
    /// - [`span_id`](OTelSpanBuilder::span_id)
    /// - [`name`](OTelSpanBuilder::name)
    /// - [`kind`](OTelSpanBuilder::kind)
    /// - [`start_time_unix_nano`](OTelSpanBuilder::start_time_unix_nano)
    /// - [`end_time_unix_nano`](OTelSpanBuilder::end_time_unix_nano)
    /// - [`attributes`](OTelSpanBuilder::attributes)
    /// - [`status`](OTelSpanBuilder::status)
    pub fn build(self) -> Result<OTelSpan, BuildError> {
        Ok(OTelSpan {
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            span_id: self.span_id.ok_or_else(|| BuildError::missing_field("span_id"))?,
            parent_span_id: self.parent_span_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            start_time_unix_nano: self.start_time_unix_nano.ok_or_else(|| BuildError::missing_field("start_time_unix_nano"))?,
            end_time_unix_nano: self.end_time_unix_nano.ok_or_else(|| BuildError::missing_field("end_time_unix_nano"))?,
            attributes: self.attributes.ok_or_else(|| BuildError::missing_field("attributes"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
