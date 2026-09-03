pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OTelScopeSpans {
    #[serde(default)]
    pub scope: OTelScope,
    #[serde(default)]
    pub spans: Vec<OTelSpan>,
}

impl OTelScopeSpans {
    pub fn builder() -> OTelScopeSpansBuilder {
        <OTelScopeSpansBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelScopeSpansBuilder {
    scope: Option<OTelScope>,
    spans: Option<Vec<OTelSpan>>,
}

impl OTelScopeSpansBuilder {
    pub fn scope(mut self, value: OTelScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn spans(mut self, value: Vec<OTelSpan>) -> Self {
        self.spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OTelScopeSpans`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scope`](OTelScopeSpansBuilder::scope)
    /// - [`spans`](OTelScopeSpansBuilder::spans)
    pub fn build(self) -> Result<OTelScopeSpans, BuildError> {
        Ok(OTelScopeSpans {
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            spans: self.spans.ok_or_else(|| BuildError::missing_field("spans"))?,
        })
    }
}
