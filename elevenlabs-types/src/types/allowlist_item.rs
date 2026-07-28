pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AllowlistItem {
    /// The hostname of the allowed origin
    #[serde(default)]
    pub hostname: String,
}

impl AllowlistItem {
    pub fn builder() -> AllowlistItemBuilder {
        <AllowlistItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AllowlistItemBuilder {
    hostname: Option<String>,
}

impl AllowlistItemBuilder {
    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AllowlistItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hostname`](AllowlistItemBuilder::hostname)
    pub fn build(self) -> Result<AllowlistItem, BuildError> {
        Ok(AllowlistItem {
            hostname: self.hostname.ok_or_else(|| BuildError::missing_field("hostname"))?,
        })
    }
}
