pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateEnvironmentVariableRequest {
    /// Values to replace. Set to null to remove an environment (except 'production').
    #[serde(default)]
    pub values: HashMap<String, Option<UpdateEnvironmentVariableRequestValuesValue>>,
}

impl UpdateEnvironmentVariableRequest {
    pub fn builder() -> UpdateEnvironmentVariableRequestBuilder {
        <UpdateEnvironmentVariableRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEnvironmentVariableRequestBuilder {
    values: Option<HashMap<String, Option<UpdateEnvironmentVariableRequestValuesValue>>>,
}

impl UpdateEnvironmentVariableRequestBuilder {
    pub fn values(mut self, value: HashMap<String, Option<UpdateEnvironmentVariableRequestValuesValue>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEnvironmentVariableRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`values`](UpdateEnvironmentVariableRequestBuilder::values)
    pub fn build(self) -> Result<UpdateEnvironmentVariableRequest, BuildError> {
        Ok(UpdateEnvironmentVariableRequest {
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}

