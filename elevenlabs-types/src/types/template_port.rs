pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One input or output port of a published template version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplatePort {
    /// The port id. Input ids are the keys of the `inputs` map on `POST /v1/flows/templates/{template_id}/runs`; output ids are the keys of the `outputs` map on the run response.
    #[serde(default)]
    pub id: String,
    /// What this port accepts or produces, as a `ContentSchema`. Its `title` is the port's display name and its `description` is the help text the template author wrote. For an input, its `type` decides what value is accepted: `string` takes a bare string or a `generation` reference, `voice` takes a `voice` reference, and `image`/`video`/`audio` take an `asset`, `generation` or `inline_base64` reference. `number`, `integer` and `boolean` take a JSON value of that type. An `array` input takes a JSON array with one value per element, each admissible for its `items`. `object` inputs cannot be bound through this API yet. A `string` schema may carry an `enum` of the only values accepted. For an output, its `type` decides which `Template<Kind>Output` shape the run response holds under the port id.
    pub content_schema: ContentSchema,
}

impl TemplatePort {
    pub fn builder() -> TemplatePortBuilder {
        <TemplatePortBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplatePortBuilder {
    id: Option<String>,
    content_schema: Option<ContentSchema>,
}

impl TemplatePortBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content_schema(mut self, value: ContentSchema) -> Self {
        self.content_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplatePort`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TemplatePortBuilder::id)
    /// - [`content_schema`](TemplatePortBuilder::content_schema)
    pub fn build(self) -> Result<TemplatePort, BuildError> {
        Ok(TemplatePort {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content_schema: self.content_schema.ok_or_else(|| BuildError::missing_field("content_schema"))?,
        })
    }
}
