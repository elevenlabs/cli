pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ShareOptionResponseModel {
    /// The name of the principal.
    #[serde(default)]
    pub name: String,
    /// The ID of the principal.
    #[serde(default)]
    pub id: String,
    /// The type of the principal: user, group, or service account (under 'key').
    pub r#type: ShareOptionResponseModelType,
}

impl ShareOptionResponseModel {
    pub fn builder() -> ShareOptionResponseModelBuilder {
        <ShareOptionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShareOptionResponseModelBuilder {
    name: Option<String>,
    id: Option<String>,
    r#type: Option<ShareOptionResponseModelType>,
}

impl ShareOptionResponseModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ShareOptionResponseModelType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShareOptionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ShareOptionResponseModelBuilder::name)
    /// - [`id`](ShareOptionResponseModelBuilder::id)
    /// - [`r#type`](ShareOptionResponseModelBuilder::r#type)
    pub fn build(self) -> Result<ShareOptionResponseModel, BuildError> {
        Ok(ShareOptionResponseModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
