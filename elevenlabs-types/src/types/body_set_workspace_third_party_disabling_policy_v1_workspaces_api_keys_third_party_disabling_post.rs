pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPost {
    /// `true` forces every key in the workspace to be disable-able by its holder; `false` forbids it for every key; `null` clears the override (per-key values and the plan default apply).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_disable_allowed: Option<bool>,
}

impl BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPost {
    pub fn builder() -> BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPostBuilder {
        <BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPostBuilder {
    third_party_disable_allowed: Option<bool>,
}

impl BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPostBuilder {
    pub fn third_party_disable_allowed(mut self, value: bool) -> Self {
        self.third_party_disable_allowed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPost`].
    pub fn build(self) -> Result<BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPost, BuildError> {
        Ok(BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPost {
            third_party_disable_allowed: self.third_party_disable_allowed,
        })
    }
}

