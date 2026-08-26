pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignableUserResponseModel {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default)]
    pub is_service_account: bool,
    /// Whether this workspace member currently has at least viewer access to the agent. Members without access are still returned so they can be surfaced (e.g. grayed out) and granted access before being assigned.
    #[serde(default)]
    pub has_access: bool,
}

impl AssignableUserResponseModel {
    pub fn builder() -> AssignableUserResponseModelBuilder {
        <AssignableUserResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignableUserResponseModelBuilder {
    user_id: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    is_service_account: Option<bool>,
    has_access: Option<bool>,
}

impl AssignableUserResponseModelBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn is_service_account(mut self, value: bool) -> Self {
        self.is_service_account = Some(value);
        self
    }

    pub fn has_access(mut self, value: bool) -> Self {
        self.has_access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssignableUserResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](AssignableUserResponseModelBuilder::user_id)
    /// - [`email`](AssignableUserResponseModelBuilder::email)
    /// - [`is_service_account`](AssignableUserResponseModelBuilder::is_service_account)
    /// - [`has_access`](AssignableUserResponseModelBuilder::has_access)
    pub fn build(self) -> Result<AssignableUserResponseModel, BuildError> {
        Ok(AssignableUserResponseModel {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            is_service_account: self.is_service_account.ok_or_else(|| BuildError::missing_field("is_service_account"))?,
            has_access: self.has_access.ok_or_else(|| BuildError::missing_field("has_access"))?,
        })
    }
}
