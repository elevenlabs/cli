pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions {
        PermissionTypeList(Vec<PermissionType>),

        String(String),
}

impl BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions {
    pub fn is_permission_type_list(&self) -> bool {
        matches!(self, Self::PermissionTypeList(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_permission_type_list(&self) -> Option<&Vec<PermissionType>> {
        match self {
                    Self::PermissionTypeList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_permission_type_list(self) -> Option<Vec<PermissionType>> {
        match self {
                    Self::PermissionTypeList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }
}
