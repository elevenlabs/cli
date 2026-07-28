pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// OCSF URL object.
/// 
/// Spec: https://schema.ocsf.io/1.6.0/objects/url
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UrlModel {
    /// Full URL string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_string: Option<String>,
    /// URL scheme (e.g., https)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// URL hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// URL port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// URL path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// URL query string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
}

impl UrlModel {
    pub fn builder() -> UrlModelBuilder {
        <UrlModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UrlModelBuilder {
    url_string: Option<String>,
    scheme: Option<String>,
    hostname: Option<String>,
    port: Option<i64>,
    path: Option<String>,
    query_string: Option<String>,
}

impl UrlModelBuilder {
    pub fn url_string(mut self, value: impl Into<String>) -> Self {
        self.url_string = Some(value.into());
        self
    }

    pub fn scheme(mut self, value: impl Into<String>) -> Self {
        self.scheme = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn query_string(mut self, value: impl Into<String>) -> Self {
        self.query_string = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UrlModel`].
    pub fn build(self) -> Result<UrlModel, BuildError> {
        Ok(UrlModel {
            url_string: self.url_string,
            scheme: self.scheme,
            hostname: self.hostname,
            port: self.port,
            path: self.path,
            query_string: self.query_string,
        })
    }
}
