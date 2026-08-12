pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetSecretDependenciesResponseModel {
    pub dependencies: GetSecretDependenciesResponseModelDependencies,
    /// Cursor for fetching the next page of dependencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl GetSecretDependenciesResponseModel {
    pub fn builder() -> GetSecretDependenciesResponseModelBuilder {
        <GetSecretDependenciesResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSecretDependenciesResponseModelBuilder {
    dependencies: Option<GetSecretDependenciesResponseModelDependencies>,
    next_cursor: Option<String>,
}

impl GetSecretDependenciesResponseModelBuilder {
    pub fn dependencies(mut self, value: GetSecretDependenciesResponseModelDependencies) -> Self {
        self.dependencies = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSecretDependenciesResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dependencies`](GetSecretDependenciesResponseModelBuilder::dependencies)
    pub fn build(self) -> Result<GetSecretDependenciesResponseModel, BuildError> {
        Ok(GetSecretDependenciesResponseModel {
            dependencies: self.dependencies.ok_or_else(|| BuildError::missing_field("dependencies"))?,
            next_cursor: self.next_cursor,
        })
    }
}
