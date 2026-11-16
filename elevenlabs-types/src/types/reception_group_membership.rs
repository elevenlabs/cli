pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Membership of an item in a single user-defined group, with its display rank inside that group.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionGroupMembership {
    /// Name of the group.
    #[serde(default)]
    pub label: StrippedStr,
    /// 0-based display position within this group. Change it with the reorder endpoint; send back the value you read to keep the order, a membership sent without it goes to position 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

impl ReceptionGroupMembership {
    pub fn builder() -> ReceptionGroupMembershipBuilder {
        <ReceptionGroupMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionGroupMembershipBuilder {
    label: Option<StrippedStr>,
    rank: Option<i64>,
}

impl ReceptionGroupMembershipBuilder {
    pub fn label(mut self, value: StrippedStr) -> Self {
        self.label = Some(value);
        self
    }

    pub fn rank(mut self, value: i64) -> Self {
        self.rank = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionGroupMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](ReceptionGroupMembershipBuilder::label)
    pub fn build(self) -> Result<ReceptionGroupMembership, BuildError> {
        Ok(ReceptionGroupMembership {
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            rank: self.rank,
        })
    }
}
