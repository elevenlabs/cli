pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMemberUsageLimitResponseModel {
    #[serde(default)]
    pub status: String,
}

impl UpdateMemberUsageLimitResponseModel {
    pub fn builder() -> UpdateMemberUsageLimitResponseModelBuilder {
        <UpdateMemberUsageLimitResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberUsageLimitResponseModelBuilder {
    status: Option<String>,
}

impl UpdateMemberUsageLimitResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberUsageLimitResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdateMemberUsageLimitResponseModelBuilder::status)
    pub fn build(self) -> Result<UpdateMemberUsageLimitResponseModel, BuildError> {
        Ok(UpdateMemberUsageLimitResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
