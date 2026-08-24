pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct FinalOutput {
    /// Indicates if the generation is complete. If set to `True`, `audio` will be null.
    pub is_final: Option<bool>,
}

impl FinalOutput {
    pub fn builder() -> FinalOutputBuilder {
        <FinalOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalOutputBuilder {
    is_final: Option<bool>,
}

impl FinalOutputBuilder {
    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinalOutput`].
    pub fn build(self) -> Result<FinalOutput, BuildError> {
        Ok(FinalOutput {
            is_final: self.is_final,
        })
    }
}
