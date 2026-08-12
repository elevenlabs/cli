pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BehaviorOverride {
    /// Verbosity override. Underlying default applies when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,
    /// Output format override. Underlying default applies when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<OutputFormat>,
    /// Interaction budget override. Underlying default applies when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_budget: Option<InteractionBudget>,
}

impl BehaviorOverride {
    pub fn builder() -> BehaviorOverrideBuilder {
        <BehaviorOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BehaviorOverrideBuilder {
    verbosity: Option<Verbosity>,
    output_format: Option<OutputFormat>,
    interaction_budget: Option<InteractionBudget>,
}

impl BehaviorOverrideBuilder {
    pub fn verbosity(mut self, value: Verbosity) -> Self {
        self.verbosity = Some(value);
        self
    }

    pub fn output_format(mut self, value: OutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn interaction_budget(mut self, value: InteractionBudget) -> Self {
        self.interaction_budget = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BehaviorOverride`].
    pub fn build(self) -> Result<BehaviorOverride, BuildError> {
        Ok(BehaviorOverride {
            verbosity: self.verbosity,
            output_format: self.output_format,
            interaction_budget: self.interaction_budget,
        })
    }
}
