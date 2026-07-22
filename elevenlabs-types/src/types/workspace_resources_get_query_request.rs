pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceResourcesGetQueryRequest {
    /// Resource type of the target resource.
    pub resource_type: WorkspaceResourceType,
}

impl WorkspaceResourcesGetQueryRequest {
    pub fn builder() -> WorkspaceResourcesGetQueryRequestBuilder {
        <WorkspaceResourcesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceResourcesGetQueryRequestBuilder {
    resource_type: Option<WorkspaceResourceType>,
}

impl WorkspaceResourcesGetQueryRequestBuilder {
    pub fn resource_type(mut self, value: WorkspaceResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceResourcesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_type`](WorkspaceResourcesGetQueryRequestBuilder::resource_type)
    pub fn build(self) -> Result<WorkspaceResourcesGetQueryRequest, BuildError> {
        Ok(WorkspaceResourcesGetQueryRequest {
            resource_type: self.resource_type.ok_or_else(|| BuildError::missing_field("resource_type"))?,
        })
    }
}

