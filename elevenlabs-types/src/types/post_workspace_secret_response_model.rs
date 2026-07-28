pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostWorkspaceSecretResponseModel {
    pub r#type: String,
    #[serde(default)]
    pub secret_id: String,
    #[serde(default)]
    pub name: String,
}

impl PostWorkspaceSecretResponseModel {
    pub fn builder() -> PostWorkspaceSecretResponseModelBuilder {
        <PostWorkspaceSecretResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostWorkspaceSecretResponseModelBuilder {
    r#type: Option<String>,
    secret_id: Option<String>,
    name: Option<String>,
}

impl PostWorkspaceSecretResponseModelBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn secret_id(mut self, value: impl Into<String>) -> Self {
        self.secret_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostWorkspaceSecretResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostWorkspaceSecretResponseModelBuilder::r#type)
    /// - [`secret_id`](PostWorkspaceSecretResponseModelBuilder::secret_id)
    /// - [`name`](PostWorkspaceSecretResponseModelBuilder::name)
    pub fn build(self) -> Result<PostWorkspaceSecretResponseModel, BuildError> {
        Ok(PostWorkspaceSecretResponseModel {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            secret_id: self.secret_id.ok_or_else(|| BuildError::missing_field("secret_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
