pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OTelTracePayload {
    #[serde(rename = "resourceSpans")]
    #[serde(default)]
    pub resource_spans: Vec<OTelResourceSpans>,
}

impl OTelTracePayload {
    pub fn builder() -> OTelTracePayloadBuilder {
        <OTelTracePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelTracePayloadBuilder {
    resource_spans: Option<Vec<OTelResourceSpans>>,
}

impl OTelTracePayloadBuilder {
    pub fn resource_spans(mut self, value: Vec<OTelResourceSpans>) -> Self {
        self.resource_spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OTelTracePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_spans`](OTelTracePayloadBuilder::resource_spans)
    pub fn build(self) -> Result<OTelTracePayload, BuildError> {
        Ok(OTelTracePayload {
            resource_spans: self.resource_spans.ok_or_else(|| BuildError::missing_field("resource_spans"))?,
        })
    }
}
