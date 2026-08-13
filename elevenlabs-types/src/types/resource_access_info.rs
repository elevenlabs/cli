pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResourceAccessInfo {
    /// Whether the user making the request is the creator of the agent
    #[serde(default)]
    pub is_creator: bool,
    /// Name of the agent's creator
    #[serde(default)]
    pub creator_name: String,
    /// Email of the agent's creator
    #[serde(default)]
    pub creator_email: String,
    /// The role of the user making the request
    pub role: ResourceAccessInfoRole,
    /// The access level for anonymous users. If None, the resource is not shared publicly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_access_level_override: Option<ResourceAccessInfoAnonymousAccessLevelOverride>,
    /// Why the requesting user has access to this resource. 'creator' = caller is the owner. 'explicit' = caller (or one of their workspace groups) is listed in role_to_group_ids beyond the workspace-wide everyone group. 'workspace_default' = the workspace-wide everyone group is listed in role_to_group_ids (every non-anon workspace member, including admins, sees this resource). 'workspace_admin' = caller is a workspace admin and the admin seat is the *only* path to access; reserved for docs nobody else can see. Lets the UI disclose why an admin-bypass viewer sees a doc that wasn't explicitly shared with them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_source: Option<ResourceAccessInfoAccessSource>,
}

impl ResourceAccessInfo {
    pub fn builder() -> ResourceAccessInfoBuilder {
        <ResourceAccessInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResourceAccessInfoBuilder {
    is_creator: Option<bool>,
    creator_name: Option<String>,
    creator_email: Option<String>,
    role: Option<ResourceAccessInfoRole>,
    anonymous_access_level_override: Option<ResourceAccessInfoAnonymousAccessLevelOverride>,
    access_source: Option<ResourceAccessInfoAccessSource>,
}

impl ResourceAccessInfoBuilder {
    pub fn is_creator(mut self, value: bool) -> Self {
        self.is_creator = Some(value);
        self
    }

    pub fn creator_name(mut self, value: impl Into<String>) -> Self {
        self.creator_name = Some(value.into());
        self
    }

    pub fn creator_email(mut self, value: impl Into<String>) -> Self {
        self.creator_email = Some(value.into());
        self
    }

    pub fn role(mut self, value: ResourceAccessInfoRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn anonymous_access_level_override(mut self, value: ResourceAccessInfoAnonymousAccessLevelOverride) -> Self {
        self.anonymous_access_level_override = Some(value);
        self
    }

    pub fn access_source(mut self, value: ResourceAccessInfoAccessSource) -> Self {
        self.access_source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResourceAccessInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_creator`](ResourceAccessInfoBuilder::is_creator)
    /// - [`creator_name`](ResourceAccessInfoBuilder::creator_name)
    /// - [`creator_email`](ResourceAccessInfoBuilder::creator_email)
    /// - [`role`](ResourceAccessInfoBuilder::role)
    pub fn build(self) -> Result<ResourceAccessInfo, BuildError> {
        Ok(ResourceAccessInfo {
            is_creator: self.is_creator.ok_or_else(|| BuildError::missing_field("is_creator"))?,
            creator_name: self.creator_name.ok_or_else(|| BuildError::missing_field("creator_name"))?,
            creator_email: self.creator_email.ok_or_else(|| BuildError::missing_field("creator_email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            anonymous_access_level_override: self.anonymous_access_level_override,
            access_source: self.access_source,
        })
    }
}
