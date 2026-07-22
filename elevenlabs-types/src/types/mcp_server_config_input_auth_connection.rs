pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum McpServerConfigInputAuthConnection {
        AuthConnectionLocator(AuthConnectionLocator),

        EnvironmentAuthConnectionLocator(EnvironmentAuthConnectionLocator),
}

impl McpServerConfigInputAuthConnection {
    pub fn is_auth_connection_locator(&self) -> bool {
        matches!(self, Self::AuthConnectionLocator(_))
    }

    pub fn is_environment_auth_connection_locator(&self) -> bool {
        matches!(self, Self::EnvironmentAuthConnectionLocator(_))
    }


    pub fn as_auth_connection_locator(&self) -> Option<&AuthConnectionLocator> {
        match self {
                    Self::AuthConnectionLocator(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_auth_connection_locator(self) -> Option<AuthConnectionLocator> {
        match self {
                    Self::AuthConnectionLocator(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_environment_auth_connection_locator(&self) -> Option<&EnvironmentAuthConnectionLocator> {
        match self {
                    Self::EnvironmentAuthConnectionLocator(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_environment_auth_connection_locator(self) -> Option<EnvironmentAuthConnectionLocator> {
        match self {
                    Self::EnvironmentAuthConnectionLocator(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for McpServerConfigInputAuthConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthConnectionLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::EnvironmentAuthConnectionLocator(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
