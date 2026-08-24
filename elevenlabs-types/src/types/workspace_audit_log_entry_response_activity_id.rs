pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum WorkspaceAuditLogEntryResponseActivityId {
        AccountChangeActivityId(AccountChangeActivityId),

        AuthenticationActivityId(AuthenticationActivityId),

        EntityManagementActivityId(EntityManagementActivityId),

        UserAccessManagementActivityId(UserAccessManagementActivityId),

        GroupManagementActivityId(GroupManagementActivityId),
}

impl WorkspaceAuditLogEntryResponseActivityId {
    pub fn is_account_change_activity_id(&self) -> bool {
        matches!(self, Self::AccountChangeActivityId(_))
    }

    pub fn is_authentication_activity_id(&self) -> bool {
        matches!(self, Self::AuthenticationActivityId(_))
    }

    pub fn is_entity_management_activity_id(&self) -> bool {
        matches!(self, Self::EntityManagementActivityId(_))
    }

    pub fn is_user_access_management_activity_id(&self) -> bool {
        matches!(self, Self::UserAccessManagementActivityId(_))
    }

    pub fn is_group_management_activity_id(&self) -> bool {
        matches!(self, Self::GroupManagementActivityId(_))
    }


    pub fn as_account_change_activity_id(&self) -> Option<&AccountChangeActivityId> {
        match self {
                    Self::AccountChangeActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_account_change_activity_id(self) -> Option<AccountChangeActivityId> {
        match self {
                    Self::AccountChangeActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_authentication_activity_id(&self) -> Option<&AuthenticationActivityId> {
        match self {
                    Self::AuthenticationActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_authentication_activity_id(self) -> Option<AuthenticationActivityId> {
        match self {
                    Self::AuthenticationActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_entity_management_activity_id(&self) -> Option<&EntityManagementActivityId> {
        match self {
                    Self::EntityManagementActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_entity_management_activity_id(self) -> Option<EntityManagementActivityId> {
        match self {
                    Self::EntityManagementActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_user_access_management_activity_id(&self) -> Option<&UserAccessManagementActivityId> {
        match self {
                    Self::UserAccessManagementActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_user_access_management_activity_id(self) -> Option<UserAccessManagementActivityId> {
        match self {
                    Self::UserAccessManagementActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_group_management_activity_id(&self) -> Option<&GroupManagementActivityId> {
        match self {
                    Self::GroupManagementActivityId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_group_management_activity_id(self) -> Option<GroupManagementActivityId> {
        match self {
                    Self::GroupManagementActivityId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for WorkspaceAuditLogEntryResponseActivityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountChangeActivityId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AuthenticationActivityId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::EntityManagementActivityId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UserAccessManagementActivityId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::GroupManagementActivityId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
