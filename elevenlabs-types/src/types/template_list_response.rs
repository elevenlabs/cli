pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of the templates the caller can run, most recently updated first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateListResponse {
    /// The templates on this page, most recently updated first. A page can hold fewer than `page_size` templates, or none at all, because templates you cannot run are filtered out after the page is read — keep paging while `has_more` is true.
    #[serde(default)]
    pub templates: Vec<TemplateSummary>,
    /// Pass as `cursor` to fetch the next page. `null` when there is no further page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether more templates exist beyond this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl TemplateListResponse {
    pub fn builder() -> TemplateListResponseBuilder {
        <TemplateListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateListResponseBuilder {
    templates: Option<Vec<TemplateSummary>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl TemplateListResponseBuilder {
    pub fn templates(mut self, value: Vec<TemplateSummary>) -> Self {
        self.templates = Some(value);
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

    /// Consumes the builder and constructs a [`TemplateListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`templates`](TemplateListResponseBuilder::templates)
    pub fn build(self) -> Result<TemplateListResponse, BuildError> {
        Ok(TemplateListResponse {
            templates: self.templates.ok_or_else(|| BuildError::missing_field("templates"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
