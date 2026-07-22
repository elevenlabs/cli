pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateImageParamDetails {
    #[serde(default)]
    pub link: String,
}

impl WhatsAppTemplateImageParamDetails {
    pub fn builder() -> WhatsAppTemplateImageParamDetailsBuilder {
        <WhatsAppTemplateImageParamDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateImageParamDetailsBuilder {
    link: Option<String>,
}

impl WhatsAppTemplateImageParamDetailsBuilder {
    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateImageParamDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`link`](WhatsAppTemplateImageParamDetailsBuilder::link)
    pub fn build(self) -> Result<WhatsAppTemplateImageParamDetails, BuildError> {
        Ok(WhatsAppTemplateImageParamDetails {
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
        })
    }
}
