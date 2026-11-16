pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferRuleOutput {
    /// At most one transfer configuration per telephony provider. A provider without an entry cannot use this transfer rule.
    #[serde(default)]
    pub provider_configs: Vec<TransferRuleOutputProviderConfigsItem>,
    /// When the agent should pick this transfer. Supports dynamic variables.
    #[serde(default)]
    pub condition: String,
}

impl TransferRuleOutput {
    pub fn builder() -> TransferRuleOutputBuilder {
        <TransferRuleOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferRuleOutputBuilder {
    provider_configs: Option<Vec<TransferRuleOutputProviderConfigsItem>>,
    condition: Option<String>,
}

impl TransferRuleOutputBuilder {
    pub fn provider_configs(mut self, value: Vec<TransferRuleOutputProviderConfigsItem>) -> Self {
        self.provider_configs = Some(value);
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferRuleOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider_configs`](TransferRuleOutputBuilder::provider_configs)
    /// - [`condition`](TransferRuleOutputBuilder::condition)
    pub fn build(self) -> Result<TransferRuleOutput, BuildError> {
        Ok(TransferRuleOutput {
            provider_configs: self.provider_configs.ok_or_else(|| BuildError::missing_field("provider_configs"))?,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
        })
    }
}
