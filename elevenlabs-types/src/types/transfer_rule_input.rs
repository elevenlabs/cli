pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferRuleInput {
    /// At most one transfer configuration per telephony provider. A provider without an entry cannot use this transfer rule.
    #[serde(default)]
    pub provider_configs: Vec<TransferRuleInputProviderConfigsItem>,
    /// When the agent should pick this transfer. Supports dynamic variables.
    #[serde(default)]
    pub condition: String,
}

impl TransferRuleInput {
    pub fn builder() -> TransferRuleInputBuilder {
        <TransferRuleInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferRuleInputBuilder {
    provider_configs: Option<Vec<TransferRuleInputProviderConfigsItem>>,
    condition: Option<String>,
}

impl TransferRuleInputBuilder {
    pub fn provider_configs(mut self, value: Vec<TransferRuleInputProviderConfigsItem>) -> Self {
        self.provider_configs = Some(value);
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferRuleInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider_configs`](TransferRuleInputBuilder::provider_configs)
    /// - [`condition`](TransferRuleInputBuilder::condition)
    pub fn build(self) -> Result<TransferRuleInput, BuildError> {
        Ok(TransferRuleInput {
            provider_configs: self.provider_configs.ok_or_else(|| BuildError::missing_field("provider_configs"))?,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
        })
    }
}
