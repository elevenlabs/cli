pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Wrapper for anyOf/allOf composition constraints scoped to required fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequiredConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<RequiredConstraint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<RequiredConstraint>>,
}

impl RequiredConstraints {
    pub fn builder() -> RequiredConstraintsBuilder {
        <RequiredConstraintsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredConstraintsBuilder {
    any_of: Option<Vec<RequiredConstraint>>,
    all_of: Option<Vec<RequiredConstraint>>,
}

impl RequiredConstraintsBuilder {
    pub fn any_of(mut self, value: Vec<RequiredConstraint>) -> Self {
        self.any_of = Some(value);
        self
    }

    pub fn all_of(mut self, value: Vec<RequiredConstraint>) -> Self {
        self.all_of = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredConstraints`].
    pub fn build(self) -> Result<RequiredConstraints, BuildError> {
        Ok(RequiredConstraints {
            any_of: self.any_of,
            all_of: self.all_of,
        })
    }
}
