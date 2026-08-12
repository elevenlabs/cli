pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetSecretDependenciesResponseModelDependencies {
        GetSecretDependenciesResponseModelDependenciesZeroItemList(Vec<GetSecretDependenciesResponseModelDependenciesZeroItem>),

        GetSecretDependenciesResponseModelDependenciesOneItemList(Vec<GetSecretDependenciesResponseModelDependenciesOneItem>),

        DependentPhoneNumberIdentifierList(Vec<DependentPhoneNumberIdentifier>),
}

impl GetSecretDependenciesResponseModelDependencies {
    pub fn is_get_secret_dependencies_response_model_dependencies_zero_item_list(&self) -> bool {
        matches!(self, Self::GetSecretDependenciesResponseModelDependenciesZeroItemList(_))
    }

    pub fn is_get_secret_dependencies_response_model_dependencies_one_item_list(&self) -> bool {
        matches!(self, Self::GetSecretDependenciesResponseModelDependenciesOneItemList(_))
    }

    pub fn is_dependent_phone_number_identifier_list(&self) -> bool {
        matches!(self, Self::DependentPhoneNumberIdentifierList(_))
    }


    pub fn as_get_secret_dependencies_response_model_dependencies_zero_item_list(&self) -> Option<&Vec<GetSecretDependenciesResponseModelDependenciesZeroItem>> {
        match self {
                    Self::GetSecretDependenciesResponseModelDependenciesZeroItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_get_secret_dependencies_response_model_dependencies_zero_item_list(self) -> Option<Vec<GetSecretDependenciesResponseModelDependenciesZeroItem>> {
        match self {
                    Self::GetSecretDependenciesResponseModelDependenciesZeroItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_get_secret_dependencies_response_model_dependencies_one_item_list(&self) -> Option<&Vec<GetSecretDependenciesResponseModelDependenciesOneItem>> {
        match self {
                    Self::GetSecretDependenciesResponseModelDependenciesOneItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_get_secret_dependencies_response_model_dependencies_one_item_list(self) -> Option<Vec<GetSecretDependenciesResponseModelDependenciesOneItem>> {
        match self {
                    Self::GetSecretDependenciesResponseModelDependenciesOneItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dependent_phone_number_identifier_list(&self) -> Option<&Vec<DependentPhoneNumberIdentifier>> {
        match self {
                    Self::DependentPhoneNumberIdentifierList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dependent_phone_number_identifier_list(self) -> Option<Vec<DependentPhoneNumberIdentifier>> {
        match self {
                    Self::DependentPhoneNumberIdentifierList(value) => Some(value),
                    _ => None,
                }
    }
}
