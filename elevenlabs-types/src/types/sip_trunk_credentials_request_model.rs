pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SipTrunkCredentialsRequestModel {
    /// SIP trunk username
    #[serde(default)]
    pub username: String,
    /// SIP trunk password - if not specified, then remain unchanged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl SipTrunkCredentialsRequestModel {
    pub fn builder() -> SipTrunkCredentialsRequestModelBuilder {
        <SipTrunkCredentialsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipTrunkCredentialsRequestModelBuilder {
    username: Option<String>,
    password: Option<String>,
}

impl SipTrunkCredentialsRequestModelBuilder {
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipTrunkCredentialsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`username`](SipTrunkCredentialsRequestModelBuilder::username)
    pub fn build(self) -> Result<SipTrunkCredentialsRequestModel, BuildError> {
        Ok(SipTrunkCredentialsRequestModel {
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
            password: self.password,
        })
    }
}
