pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddUrlRequest {
    /// The HTTPS page or site to read.
    #[serde(default)]
    pub url: String,
    /// Crawl the whole site instead of only the page at url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_entire_site: Option<bool>,
}

impl AddUrlRequest {
    pub fn builder() -> AddUrlRequestBuilder {
        <AddUrlRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddUrlRequestBuilder {
    url: Option<String>,
    crawl_entire_site: Option<bool>,
}

impl AddUrlRequestBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn crawl_entire_site(mut self, value: bool) -> Self {
        self.crawl_entire_site = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddUrlRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AddUrlRequestBuilder::url)
    pub fn build(self) -> Result<AddUrlRequest, BuildError> {
        Ok(AddUrlRequest {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            crawl_entire_site: self.crawl_entire_site,
        })
    }
}

