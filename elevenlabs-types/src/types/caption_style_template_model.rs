pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptionStyleTemplateModel {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_high_fps: Option<bool>,
}

impl CaptionStyleTemplateModel {
    pub fn builder() -> CaptionStyleTemplateModelBuilder {
        <CaptionStyleTemplateModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleTemplateModelBuilder {
    key: Option<String>,
    label: Option<String>,
    requires_high_fps: Option<bool>,
}

impl CaptionStyleTemplateModelBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn requires_high_fps(mut self, value: bool) -> Self {
        self.requires_high_fps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleTemplateModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](CaptionStyleTemplateModelBuilder::key)
    /// - [`label`](CaptionStyleTemplateModelBuilder::label)
    pub fn build(self) -> Result<CaptionStyleTemplateModel, BuildError> {
        Ok(CaptionStyleTemplateModel {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            requires_high_fps: self.requires_high_fps,
        })
    }
}
