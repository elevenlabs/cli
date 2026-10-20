pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentsListResponse {
    /// All workspace environments, 'production' first. Variables without a value for an environment fall back to their production value.
    #[serde(default)]
    pub environments: Vec<EnvironmentResponse>,
}

impl EnvironmentsListResponse {
    pub fn builder() -> EnvironmentsListResponseBuilder {
        <EnvironmentsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentsListResponseBuilder {
    environments: Option<Vec<EnvironmentResponse>>,
}

impl EnvironmentsListResponseBuilder {
    pub fn environments(mut self, value: Vec<EnvironmentResponse>) -> Self {
        self.environments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environments`](EnvironmentsListResponseBuilder::environments)
    pub fn build(self) -> Result<EnvironmentsListResponse, BuildError> {
        Ok(EnvironmentsListResponse {
            environments: self.environments.ok_or_else(|| BuildError::missing_field("environments"))?,
        })
    }
}
