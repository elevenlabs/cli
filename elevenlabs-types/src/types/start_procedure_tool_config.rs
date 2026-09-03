pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StartProcedureToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedures: Option<HashMap<String, StartProcedureToolConfigProceduresValue>>,
}

impl StartProcedureToolConfig {
    pub fn builder() -> StartProcedureToolConfigBuilder {
        <StartProcedureToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartProcedureToolConfigBuilder {
    procedures: Option<HashMap<String, StartProcedureToolConfigProceduresValue>>,
}

impl StartProcedureToolConfigBuilder {
    pub fn procedures(mut self, value: HashMap<String, StartProcedureToolConfigProceduresValue>) -> Self {
        self.procedures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StartProcedureToolConfig`].
    pub fn build(self) -> Result<StartProcedureToolConfig, BuildError> {
        Ok(StartProcedureToolConfig {
            procedures: self.procedures,
        })
    }
}
