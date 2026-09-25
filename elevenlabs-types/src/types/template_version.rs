pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A published snapshot of a template, and the ports it runs with.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateVersion {
    /// Pass as `version_id` on `POST /v1/flows/templates/{template_id}/runs` to pin a run to this snapshot.
    #[serde(default)]
    pub version_id: String,
    /// When this version was published, as a Unix timestamp in seconds.
    #[serde(default)]
    pub published_at_unix: i64,
    /// Whether this is the version a run gets when `version_id` is omitted or set to `latest`.
    #[serde(default)]
    pub is_latest: bool,
    /// The inputs this version accepts, in canvas order. Every input is required on a run.
    #[serde(default)]
    pub inputs: Vec<TemplatePort>,
    /// The outputs this version produces, in canvas order.
    #[serde(default)]
    pub outputs: Vec<TemplatePort>,
}

impl TemplateVersion {
    pub fn builder() -> TemplateVersionBuilder {
        <TemplateVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateVersionBuilder {
    version_id: Option<String>,
    published_at_unix: Option<i64>,
    is_latest: Option<bool>,
    inputs: Option<Vec<TemplatePort>>,
    outputs: Option<Vec<TemplatePort>>,
}

impl TemplateVersionBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn published_at_unix(mut self, value: i64) -> Self {
        self.published_at_unix = Some(value);
        self
    }

    pub fn is_latest(mut self, value: bool) -> Self {
        self.is_latest = Some(value);
        self
    }

    pub fn inputs(mut self, value: Vec<TemplatePort>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn outputs(mut self, value: Vec<TemplatePort>) -> Self {
        self.outputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version_id`](TemplateVersionBuilder::version_id)
    /// - [`published_at_unix`](TemplateVersionBuilder::published_at_unix)
    /// - [`is_latest`](TemplateVersionBuilder::is_latest)
    /// - [`inputs`](TemplateVersionBuilder::inputs)
    /// - [`outputs`](TemplateVersionBuilder::outputs)
    pub fn build(self) -> Result<TemplateVersion, BuildError> {
        Ok(TemplateVersion {
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
            published_at_unix: self.published_at_unix.ok_or_else(|| BuildError::missing_field("published_at_unix"))?,
            is_latest: self.is_latest.ok_or_else(|| BuildError::missing_field("is_latest"))?,
            inputs: self.inputs.ok_or_else(|| BuildError::missing_field("inputs"))?,
            outputs: self.outputs.ok_or_else(|| BuildError::missing_field("outputs"))?,
        })
    }
}
