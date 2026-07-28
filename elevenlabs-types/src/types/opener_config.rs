pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OpenerConfig {
    /// Model used to speak a fast opener while the main model generates its full reply. Must be a hosted model (not a bring-your-own LLM type).
    pub llm: Llm,
}

impl OpenerConfig {
    pub fn builder() -> OpenerConfigBuilder {
        <OpenerConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenerConfigBuilder {
    llm: Option<Llm>,
}

impl OpenerConfigBuilder {
    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenerConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llm`](OpenerConfigBuilder::llm)
    pub fn build(self) -> Result<OpenerConfig, BuildError> {
        Ok(OpenerConfig {
            llm: self.llm.ok_or_else(|| BuildError::missing_field("llm"))?,
        })
    }
}
