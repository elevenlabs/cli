pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// HTTP request details.
/// 
/// Spec: https://schema.ocsf.io/1.6.0/objects/http_request
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpRequestModel {
    /// HTTP method (GET, POST, etc.)
    #[serde(default)]
    pub http_method: String,
    /// Request URL object
    #[serde(default)]
    pub url: UrlModel,
    /// User agent string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// X-Forwarded-For header as a list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_forwarded_for: Option<Vec<String>>,
}

impl HttpRequestModel {
    pub fn builder() -> HttpRequestModelBuilder {
        <HttpRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpRequestModelBuilder {
    http_method: Option<String>,
    url: Option<UrlModel>,
    user_agent: Option<String>,
    x_forwarded_for: Option<Vec<String>>,
}

impl HttpRequestModelBuilder {
    pub fn http_method(mut self, value: impl Into<String>) -> Self {
        self.http_method = Some(value.into());
        self
    }

    pub fn url(mut self, value: UrlModel) -> Self {
        self.url = Some(value);
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn x_forwarded_for(mut self, value: Vec<String>) -> Self {
        self.x_forwarded_for = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HttpRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`http_method`](HttpRequestModelBuilder::http_method)
    /// - [`url`](HttpRequestModelBuilder::url)
    pub fn build(self) -> Result<HttpRequestModel, BuildError> {
        Ok(HttpRequestModel {
            http_method: self.http_method.ok_or_else(|| BuildError::missing_field("http_method"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            user_agent: self.user_agent,
            x_forwarded_for: self.x_forwarded_for,
        })
    }
}
