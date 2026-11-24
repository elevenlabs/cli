pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnyOfJsonSchemaPropertyOutput {
    /// The object shapes the value can take.
    #[serde(default)]
    pub any_of: Vec<Box<ObjectJsonSchemaPropertyOutput>>,
    /// The property every branch declares as a required string with a single enum value; that value names the branch.
    #[serde(default)]
    pub discriminator: String,
    /// What the value is, for the LLM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AnyOfJsonSchemaPropertyOutput {
    pub fn builder() -> AnyOfJsonSchemaPropertyOutputBuilder {
        <AnyOfJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnyOfJsonSchemaPropertyOutputBuilder {
    any_of: Option<Vec<Box<ObjectJsonSchemaPropertyOutput>>>,
    discriminator: Option<String>,
    description: Option<String>,
}

impl AnyOfJsonSchemaPropertyOutputBuilder {
    pub fn any_of(mut self, value: Vec<Box<ObjectJsonSchemaPropertyOutput>>) -> Self {
        self.any_of = Some(value);
        self
    }

    pub fn discriminator(mut self, value: impl Into<String>) -> Self {
        self.discriminator = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnyOfJsonSchemaPropertyOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`any_of`](AnyOfJsonSchemaPropertyOutputBuilder::any_of)
    /// - [`discriminator`](AnyOfJsonSchemaPropertyOutputBuilder::discriminator)
    pub fn build(self) -> Result<AnyOfJsonSchemaPropertyOutput, BuildError> {
        Ok(AnyOfJsonSchemaPropertyOutput {
            any_of: self.any_of.ok_or_else(|| BuildError::missing_field("any_of"))?,
            discriminator: self.discriminator.ok_or_else(|| BuildError::missing_field("discriminator"))?,
            description: self.description,
        })
    }
}
