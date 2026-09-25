pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A template the caller can run, and its runnable versions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateSummary {
    /// Pass as `template_id` on `POST /v1/flows/templates/{template_id}/runs`.
    #[serde(default)]
    pub id: String,
    /// The template's name.
    #[serde(default)]
    pub name: String,
    /// The template's description, if it has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The published versions this caller can run, newest first. A version whose graph uses a model that is not available to you through the API is left out, as is one whose stored snapshot is gone; either way the list can be empty while the template still has versions the ElevenLabs app can run.
    #[serde(default)]
    pub versions: Vec<TemplateVersion>,
    /// Whether this template has further published versions beyond the `versions_per_template` returned here. Fetch `GET /v1/flows/templates/{template_id}` with a larger `versions_per_template` to see more of them.
    #[serde(default)]
    pub has_more_versions: bool,
}

impl TemplateSummary {
    pub fn builder() -> TemplateSummaryBuilder {
        <TemplateSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateSummaryBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    versions: Option<Vec<TemplateVersion>>,
    has_more_versions: Option<bool>,
}

impl TemplateSummaryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn versions(mut self, value: Vec<TemplateVersion>) -> Self {
        self.versions = Some(value);
        self
    }

    pub fn has_more_versions(mut self, value: bool) -> Self {
        self.has_more_versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplateSummaryBuilder::id)
    /// - [`name`](TemplateSummaryBuilder::name)
    /// - [`versions`](TemplateSummaryBuilder::versions)
    /// - [`has_more_versions`](TemplateSummaryBuilder::has_more_versions)
    pub fn build(self) -> Result<TemplateSummary, BuildError> {
        Ok(TemplateSummary {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
            has_more_versions: self.has_more_versions.ok_or_else(|| BuildError::missing_field("has_more_versions"))?,
        })
    }
}
