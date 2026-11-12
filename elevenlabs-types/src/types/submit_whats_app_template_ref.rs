pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Names the approved Meta template to send. The server loads the template
/// itself, so the client never supplies its components.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitWhatsAppTemplateRef {
    #[serde(default)]
    pub name: String,
    /// The template's language code, such as `en_US`.
    #[serde(default)]
    pub language: String,
}

impl SubmitWhatsAppTemplateRef {
    pub fn builder() -> SubmitWhatsAppTemplateRefBuilder {
        <SubmitWhatsAppTemplateRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitWhatsAppTemplateRefBuilder {
    name: Option<String>,
    language: Option<String>,
}

impl SubmitWhatsAppTemplateRefBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitWhatsAppTemplateRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SubmitWhatsAppTemplateRefBuilder::name)
    /// - [`language`](SubmitWhatsAppTemplateRefBuilder::language)
    pub fn build(self) -> Result<SubmitWhatsAppTemplateRef, BuildError> {
        Ok(SubmitWhatsAppTemplateRef {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
        })
    }
}
