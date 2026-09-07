pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AllowedValues {
    /// Name of a dynamic variable that must resolve to a JSON array of permitted values, e.g. ["ws_alpha", "ws_beta"]. System variables work only if they resolve to a list.
    #[serde(default)]
    pub dynamic_variable: String,
}

impl AllowedValues {
    pub fn builder() -> AllowedValuesBuilder {
        <AllowedValuesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AllowedValuesBuilder {
    dynamic_variable: Option<String>,
}

impl AllowedValuesBuilder {
    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AllowedValues`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dynamic_variable`](AllowedValuesBuilder::dynamic_variable)
    pub fn build(self) -> Result<AllowedValues, BuildError> {
        Ok(AllowedValues {
            dynamic_variable: self.dynamic_variable.ok_or_else(|| BuildError::missing_field("dynamic_variable"))?,
        })
    }
}
