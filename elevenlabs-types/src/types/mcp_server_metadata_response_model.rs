pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServerMetadataResponseModel {
    #[serde(default)]
    pub created_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
}

impl McpServerMetadataResponseModel {
    pub fn builder() -> McpServerMetadataResponseModelBuilder {
        <McpServerMetadataResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerMetadataResponseModelBuilder {
    created_at: Option<i64>,
    owner_user_id: Option<String>,
}

impl McpServerMetadataResponseModelBuilder {
    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServerMetadataResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](McpServerMetadataResponseModelBuilder::created_at)
    pub fn build(self) -> Result<McpServerMetadataResponseModel, BuildError> {
        Ok(McpServerMetadataResponseModel {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            owner_user_id: self.owner_user_id,
        })
    }
}
