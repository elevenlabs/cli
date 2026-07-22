pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnvironmentVariableResponseValues {
        StringToStringMap(HashMap<String, String>),

        StringToEnvironmentVariableSecretValueMap(HashMap<String, EnvironmentVariableSecretValue>),

        StringToEnvironmentVariableAuthConnectionValueMap(HashMap<String, EnvironmentVariableAuthConnectionValue>),
}

impl EnvironmentVariableResponseValues {
    pub fn is_string_to_string_map(&self) -> bool {
        matches!(self, Self::StringToStringMap(_))
    }

    pub fn is_string_to_environment_variable_secret_value_map(&self) -> bool {
        matches!(self, Self::StringToEnvironmentVariableSecretValueMap(_))
    }

    pub fn is_string_to_environment_variable_auth_connection_value_map(&self) -> bool {
        matches!(self, Self::StringToEnvironmentVariableAuthConnectionValueMap(_))
    }


    pub fn as_string_to_string_map(&self) -> Option<&HashMap<String, String>> {
        match self {
                    Self::StringToStringMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_string_map(self) -> Option<HashMap<String, String>> {
        match self {
                    Self::StringToStringMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_environment_variable_secret_value_map(&self) -> Option<&HashMap<String, EnvironmentVariableSecretValue>> {
        match self {
                    Self::StringToEnvironmentVariableSecretValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_environment_variable_secret_value_map(self) -> Option<HashMap<String, EnvironmentVariableSecretValue>> {
        match self {
                    Self::StringToEnvironmentVariableSecretValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string_to_environment_variable_auth_connection_value_map(&self) -> Option<&HashMap<String, EnvironmentVariableAuthConnectionValue>> {
        match self {
                    Self::StringToEnvironmentVariableAuthConnectionValueMap(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_environment_variable_auth_connection_value_map(self) -> Option<HashMap<String, EnvironmentVariableAuthConnectionValue>> {
        match self {
                    Self::StringToEnvironmentVariableAuthConnectionValueMap(value) => Some(value),
                    _ => None,
                }
    }
}
