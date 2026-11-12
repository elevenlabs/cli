pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMemberUsageLimitRequestModel {
    /// Monthly credit usage limit for the member. Set to null to remove the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit: Option<i64>,
}

impl UpdateMemberUsageLimitRequestModel {
    pub fn builder() -> UpdateMemberUsageLimitRequestModelBuilder {
        <UpdateMemberUsageLimitRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberUsageLimitRequestModelBuilder {
    usage_limit: Option<i64>,
}

impl UpdateMemberUsageLimitRequestModelBuilder {
    pub fn usage_limit(mut self, value: i64) -> Self {
        self.usage_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberUsageLimitRequestModel`].
    pub fn build(self) -> Result<UpdateMemberUsageLimitRequestModel, BuildError> {
        Ok(UpdateMemberUsageLimitRequestModel {
            usage_limit: self.usage_limit,
        })
    }
}

