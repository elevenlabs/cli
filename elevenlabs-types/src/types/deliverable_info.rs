pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliverableInfo {
    /// A time-limited URL to download the delivered file.
    #[serde(default)]
    pub signed_url: String,
    /// The MIME type of the delivered file (e.g. 'video/mp4').
    #[serde(default)]
    pub content_type: String,
    /// The name of the delivered file.
    #[serde(default)]
    pub name: String,
    /// The version number of the deliverable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl DeliverableInfo {
    pub fn builder() -> DeliverableInfoBuilder {
        <DeliverableInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliverableInfoBuilder {
    signed_url: Option<String>,
    content_type: Option<String>,
    name: Option<String>,
    version: Option<i64>,
}

impl DeliverableInfoBuilder {
    pub fn signed_url(mut self, value: impl Into<String>) -> Self {
        self.signed_url = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliverableInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signed_url`](DeliverableInfoBuilder::signed_url)
    /// - [`content_type`](DeliverableInfoBuilder::content_type)
    /// - [`name`](DeliverableInfoBuilder::name)
    pub fn build(self) -> Result<DeliverableInfo, BuildError> {
        Ok(DeliverableInfo {
            signed_url: self.signed_url.ok_or_else(|| BuildError::missing_field("signed_url"))?,
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            version: self.version,
        })
    }
}
