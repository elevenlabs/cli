pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Contributor {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
}

impl Contributor {
    pub fn builder() -> ContributorBuilder {
        <ContributorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContributorBuilder {
    name: Option<String>,
    role: Option<String>,
    bio: Option<String>,
    profile_id: Option<String>,
}

impl ContributorBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn bio(mut self, value: impl Into<String>) -> Self {
        self.bio = Some(value.into());
        self
    }

    pub fn profile_id(mut self, value: impl Into<String>) -> Self {
        self.profile_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Contributor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ContributorBuilder::name)
    /// - [`role`](ContributorBuilder::role)
    pub fn build(self) -> Result<Contributor, BuildError> {
        Ok(Contributor {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            bio: self.bio,
            profile_id: self.profile_id,
        })
    }
}
