pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReaderResourceResponseModel {
    /// The type of resource.
    pub resource_type: ReaderResourceResponseModelResourceType,
    /// The ID of the resource.
    #[serde(default)]
    pub resource_id: String,
}

impl ReaderResourceResponseModel {
    pub fn builder() -> ReaderResourceResponseModelBuilder {
        <ReaderResourceResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReaderResourceResponseModelBuilder {
    resource_type: Option<ReaderResourceResponseModelResourceType>,
    resource_id: Option<String>,
}

impl ReaderResourceResponseModelBuilder {
    pub fn resource_type(mut self, value: ReaderResourceResponseModelResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReaderResourceResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_type`](ReaderResourceResponseModelBuilder::resource_type)
    /// - [`resource_id`](ReaderResourceResponseModelBuilder::resource_id)
    pub fn build(self) -> Result<ReaderResourceResponseModel, BuildError> {
        Ok(ReaderResourceResponseModel {
            resource_type: self.resource_type.ok_or_else(|| BuildError::missing_field("resource_type"))?,
            resource_id: self.resource_id.ok_or_else(|| BuildError::missing_field("resource_id"))?,
        })
    }
}
