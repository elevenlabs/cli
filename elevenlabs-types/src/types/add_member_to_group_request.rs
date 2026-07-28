pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddMemberToGroupRequest {
    /// The email of the target workspace member.
    #[serde(default)]
    pub email: String,
}

impl AddMemberToGroupRequest {
    pub fn builder() -> AddMemberToGroupRequestBuilder {
        <AddMemberToGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddMemberToGroupRequestBuilder {
    email: Option<String>,
}

impl AddMemberToGroupRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddMemberToGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](AddMemberToGroupRequestBuilder::email)
    pub fn build(self) -> Result<AddMemberToGroupRequest, BuildError> {
        Ok(AddMemberToGroupRequest {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
        })
    }
}

