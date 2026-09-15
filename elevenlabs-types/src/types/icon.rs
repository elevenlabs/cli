pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An optionally-sized icon for display in a user interface (2025-11-25+).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Icon {
    #[serde(default)]
    pub src: String,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<IconTheme>,
}

impl Icon {
    pub fn builder() -> IconBuilder {
        <IconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IconBuilder {
    src: Option<String>,
    mime_type: Option<String>,
    sizes: Option<Vec<String>>,
    theme: Option<IconTheme>,
}

impl IconBuilder {
    pub fn src(mut self, value: impl Into<String>) -> Self {
        self.src = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn sizes(mut self, value: Vec<String>) -> Self {
        self.sizes = Some(value);
        self
    }

    pub fn theme(mut self, value: IconTheme) -> Self {
        self.theme = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Icon`].
    /// This method will fail if any of the following fields are not set:
    /// - [`src`](IconBuilder::src)
    pub fn build(self) -> Result<Icon, BuildError> {
        Ok(Icon {
            src: self.src.ok_or_else(|| BuildError::missing_field("src"))?,
            mime_type: self.mime_type,
            sizes: self.sizes,
            theme: self.theme,
        })
    }
}
