pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateRunCreateRequest {
    /// Input values keyed by input port id. Every input port of the version being run must be given; a missing or unknown id is rejected. Pass `{}` for a template with no inputs.
    #[serde(default)]
    pub inputs: HashMap<String, TemplateRunInput>,
    /// The template snapshot to run. Pass a specific version id to pin that snapshot, or `latest` (the default when omitted) to run the template's most recently published version. Only published versions can be pinned, except by the template's owner, who may also pin an unpublished saved snapshot to try it out before publishing. The live draft is never run through this API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Include to send the run's result to the workspace's configured flows webhooks once the run's `status` reaches `completed` or `failed`. One event for the whole run: the `flows_template_run` event's `data` matches the terminal response of `GET /v1/flows/templates/{template_id}/runs/{run_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
}

impl TemplateRunCreateRequest {
    pub fn builder() -> TemplateRunCreateRequestBuilder {
        <TemplateRunCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateRunCreateRequestBuilder {
    inputs: Option<HashMap<String, TemplateRunInput>>,
    version_id: Option<String>,
    webhook: Option<WebhookTarget>,
}

impl TemplateRunCreateRequestBuilder {
    pub fn inputs(mut self, value: HashMap<String, TemplateRunInput>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateRunCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inputs`](TemplateRunCreateRequestBuilder::inputs)
    pub fn build(self) -> Result<TemplateRunCreateRequest, BuildError> {
        Ok(TemplateRunCreateRequest {
            inputs: self.inputs.ok_or_else(|| BuildError::missing_field("inputs"))?,
            version_id: self.version_id,
            webhook: self.webhook,
        })
    }
}

