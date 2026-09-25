pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A template run and its outputs. Every output exists from the moment the
/// run is created and reports its own `status`; the run's `status` rolls them
/// up, and the run is finished once it is `completed` or `failed`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateRunResponse {
    /// The unique identifier of the run.
    #[serde(default)]
    pub id: String,
    /// The template this run executed, so a webhook consumer running several templates can tell their runs apart without keeping a run-to-template map.
    #[serde(default)]
    pub template_id: String,
    /// The template version this run executed. Resolved when the run is created, so a run started with `latest` records the concrete version it ran.
    #[serde(default)]
    pub version_id: String,
    /// The run's status, rolled up from its outputs: `pending` until an output starts, `generating` while any output is unfinished, `completed` once every output has completed, and `failed` once every output has finished and at least one failed. `completed` and `failed` are terminal: the `flows_template_run` webhook fires once the run reaches either.
    pub status: TemplateRunStatus,
    /// The run's outputs, keyed by output port id. Each is a `TemplateOutput` discriminated on `type`, the `type` of its port's `content_schema`.
    #[serde(default)]
    pub outputs: HashMap<String, TemplateOutput>,
}

impl TemplateRunResponse {
    pub fn builder() -> TemplateRunResponseBuilder {
        <TemplateRunResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateRunResponseBuilder {
    id: Option<String>,
    template_id: Option<String>,
    version_id: Option<String>,
    status: Option<TemplateRunStatus>,
    outputs: Option<HashMap<String, TemplateOutput>>,
}

impl TemplateRunResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: TemplateRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn outputs(mut self, value: HashMap<String, TemplateOutput>) -> Self {
        self.outputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateRunResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplateRunResponseBuilder::id)
    /// - [`template_id`](TemplateRunResponseBuilder::template_id)
    /// - [`version_id`](TemplateRunResponseBuilder::version_id)
    /// - [`status`](TemplateRunResponseBuilder::status)
    /// - [`outputs`](TemplateRunResponseBuilder::outputs)
    pub fn build(self) -> Result<TemplateRunResponse, BuildError> {
        Ok(TemplateRunResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            template_id: self.template_id.ok_or_else(|| BuildError::missing_field("template_id"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            outputs: self.outputs.ok_or_else(|| BuildError::missing_field("outputs"))?,
        })
    }
}
