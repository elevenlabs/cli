pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateDocumentParamDetails {
    #[serde(default)]
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl WhatsAppTemplateDocumentParamDetails {
    pub fn builder() -> WhatsAppTemplateDocumentParamDetailsBuilder {
        <WhatsAppTemplateDocumentParamDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateDocumentParamDetailsBuilder {
    link: Option<String>,
    filename: Option<String>,
}

impl WhatsAppTemplateDocumentParamDetailsBuilder {
    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateDocumentParamDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`link`](WhatsAppTemplateDocumentParamDetailsBuilder::link)
    pub fn build(self) -> Result<WhatsAppTemplateDocumentParamDetails, BuildError> {
        Ok(WhatsAppTemplateDocumentParamDetails {
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
            filename: self.filename,
        })
    }
}
