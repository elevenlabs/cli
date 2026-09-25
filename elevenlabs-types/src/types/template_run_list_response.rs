pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of the caller's public-API template runs, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateRunListResponse {
    /// The runs on this page, newest first. Each item has the same shape as `GET /v1/flows/templates/{template_id}/runs/{run_id}`.
    #[serde(default)]
    pub runs: Vec<TemplateRunResponse>,
    /// Pass as `cursor` to fetch the next page. `null` when there is no further page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether more runs exist beyond this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl TemplateRunListResponse {
    pub fn builder() -> TemplateRunListResponseBuilder {
        <TemplateRunListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateRunListResponseBuilder {
    runs: Option<Vec<TemplateRunResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl TemplateRunListResponseBuilder {
    pub fn runs(mut self, value: Vec<TemplateRunResponse>) -> Self {
        self.runs = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateRunListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`runs`](TemplateRunListResponseBuilder::runs)
    pub fn build(self) -> Result<TemplateRunListResponse, BuildError> {
        Ok(TemplateRunListResponse {
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
