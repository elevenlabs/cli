pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingRenderResponseModel {
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub render_id: String,
}

impl DubbingRenderResponseModel {
    pub fn builder() -> DubbingRenderResponseModelBuilder {
        <DubbingRenderResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingRenderResponseModelBuilder {
    version: Option<i64>,
    render_id: Option<String>,
}

impl DubbingRenderResponseModelBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn render_id(mut self, value: impl Into<String>) -> Self {
        self.render_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingRenderResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](DubbingRenderResponseModelBuilder::version)
    /// - [`render_id`](DubbingRenderResponseModelBuilder::render_id)
    pub fn build(self) -> Result<DubbingRenderResponseModel, BuildError> {
        Ok(DubbingRenderResponseModel {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            render_id: self.render_id.ok_or_else(|| BuildError::missing_field("render_id"))?,
        })
    }
}
