pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// OCSF User object.
/// 
/// Spec: https://schema.ocsf.io/1.6.0/objects/user
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserModel {
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unique user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// Account type identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<UserTypeId>,
    /// Account type description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// User email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addr: Option<String>,
    /// Full name of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// User's domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl UserModel {
    pub fn builder() -> UserModelBuilder {
        <UserModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserModelBuilder {
    name: Option<String>,
    uid: Option<String>,
    type_id: Option<UserTypeId>,
    r#type: Option<String>,
    email_addr: Option<String>,
    full_name: Option<String>,
    domain: Option<String>,
}

impl UserModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn uid(mut self, value: impl Into<String>) -> Self {
        self.uid = Some(value.into());
        self
    }

    pub fn type_id(mut self, value: UserTypeId) -> Self {
        self.type_id = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn email_addr(mut self, value: impl Into<String>) -> Self {
        self.email_addr = Some(value.into());
        self
    }

    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserModel`].
    pub fn build(self) -> Result<UserModel, BuildError> {
        Ok(UserModel {
            name: self.name,
            uid: self.uid,
            type_id: self.type_id,
            r#type: self.r#type,
            email_addr: self.email_addr,
            full_name: self.full_name,
            domain: self.domain,
        })
    }
}
