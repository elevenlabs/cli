pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Render {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RenderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ref: Option<DubbingMediaReference>,
    pub status: RenderStatus,
}

impl Render {
    pub fn builder() -> RenderBuilder {
        <RenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenderBuilder {
    id: Option<String>,
    version: Option<i64>,
    language: Option<String>,
    r#type: Option<RenderType>,
    media_ref: Option<DubbingMediaReference>,
    status: Option<RenderStatus>,
}

impl RenderBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: RenderType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn media_ref(mut self, value: DubbingMediaReference) -> Self {
        self.media_ref = Some(value);
        self
    }

    pub fn status(mut self, value: RenderStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Render`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RenderBuilder::id)
    /// - [`version`](RenderBuilder::version)
    /// - [`status`](RenderBuilder::status)
    pub fn build(self) -> Result<Render, BuildError> {
        Ok(Render {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            language: self.language,
            r#type: self.r#type,
            media_ref: self.media_ref,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
