pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OTelResourceSpans {
    #[serde(default)]
    pub resource: OTelResource,
    #[serde(rename = "scopeSpans")]
    #[serde(default)]
    pub scope_spans: Vec<OTelScopeSpans>,
}

impl OTelResourceSpans {
    pub fn builder() -> OTelResourceSpansBuilder {
        <OTelResourceSpansBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelResourceSpansBuilder {
    resource: Option<OTelResource>,
    scope_spans: Option<Vec<OTelScopeSpans>>,
}

impl OTelResourceSpansBuilder {
    pub fn resource(mut self, value: OTelResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn scope_spans(mut self, value: Vec<OTelScopeSpans>) -> Self {
        self.scope_spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OTelResourceSpans`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource`](OTelResourceSpansBuilder::resource)
    /// - [`scope_spans`](OTelResourceSpansBuilder::scope_spans)
    pub fn build(self) -> Result<OTelResourceSpans, BuildError> {
        Ok(OTelResourceSpans {
            resource: self.resource.ok_or_else(|| BuildError::missing_field("resource"))?,
            scope_spans: self.scope_spans.ok_or_else(|| BuildError::missing_field("scope_spans"))?,
        })
    }
}
