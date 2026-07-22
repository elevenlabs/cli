pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PatchWorkspaceSecretRequest {
    pub r#type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl PatchWorkspaceSecretRequest {
    pub fn builder() -> PatchWorkspaceSecretRequestBuilder {
        <PatchWorkspaceSecretRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchWorkspaceSecretRequestBuilder {
    r#type: Option<String>,
    name: Option<String>,
    value: Option<String>,
}

impl PatchWorkspaceSecretRequestBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchWorkspaceSecretRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PatchWorkspaceSecretRequestBuilder::r#type)
    /// - [`name`](PatchWorkspaceSecretRequestBuilder::name)
    /// - [`value`](PatchWorkspaceSecretRequestBuilder::value)
    pub fn build(self) -> Result<PatchWorkspaceSecretRequest, BuildError> {
        Ok(PatchWorkspaceSecretRequest {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

