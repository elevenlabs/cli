pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A set of fields that must all be present to satisfy this constraint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequiredConstraint {
    #[serde(default)]
    pub required: Vec<String>,
}

impl RequiredConstraint {
    pub fn builder() -> RequiredConstraintBuilder {
        <RequiredConstraintBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredConstraintBuilder {
    required: Option<Vec<String>>,
}

impl RequiredConstraintBuilder {
    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredConstraint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](RequiredConstraintBuilder::required)
    pub fn build(self) -> Result<RequiredConstraint, BuildError> {
        Ok(RequiredConstraint {
            required: self.required.ok_or_else(|| BuildError::missing_field("required"))?,
        })
    }
}
