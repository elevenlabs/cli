pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitTestToolCallParameter {
    pub eval: UnitTestToolCallParameterEval,
    #[serde(default)]
    pub path: String,
}

impl UnitTestToolCallParameter {
    pub fn builder() -> UnitTestToolCallParameterBuilder {
        <UnitTestToolCallParameterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitTestToolCallParameterBuilder {
    eval: Option<UnitTestToolCallParameterEval>,
    path: Option<String>,
}

impl UnitTestToolCallParameterBuilder {
    pub fn eval(mut self, value: UnitTestToolCallParameterEval) -> Self {
        self.eval = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitTestToolCallParameter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`eval`](UnitTestToolCallParameterBuilder::eval)
    /// - [`path`](UnitTestToolCallParameterBuilder::path)
    pub fn build(self) -> Result<UnitTestToolCallParameter, BuildError> {
        Ok(UnitTestToolCallParameter {
            eval: self.eval.ok_or_else(|| BuildError::missing_field("eval"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
