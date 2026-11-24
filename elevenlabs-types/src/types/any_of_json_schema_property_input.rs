pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnyOfJsonSchemaPropertyInput {
    /// The object shapes the value can take.
    #[serde(default)]
    pub any_of: Vec<Box<ObjectJsonSchemaPropertyInput>>,
    /// The property every branch declares as a required string with a single enum value; that value names the branch.
    #[serde(default)]
    pub discriminator: String,
    /// What the value is, for the LLM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AnyOfJsonSchemaPropertyInput {
    pub fn builder() -> AnyOfJsonSchemaPropertyInputBuilder {
        <AnyOfJsonSchemaPropertyInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnyOfJsonSchemaPropertyInputBuilder {
    any_of: Option<Vec<Box<ObjectJsonSchemaPropertyInput>>>,
    discriminator: Option<String>,
    description: Option<String>,
}

impl AnyOfJsonSchemaPropertyInputBuilder {
    pub fn any_of(mut self, value: Vec<Box<ObjectJsonSchemaPropertyInput>>) -> Self {
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

    /// Consumes the builder and constructs a [`AnyOfJsonSchemaPropertyInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`any_of`](AnyOfJsonSchemaPropertyInputBuilder::any_of)
    /// - [`discriminator`](AnyOfJsonSchemaPropertyInputBuilder::discriminator)
    pub fn build(self) -> Result<AnyOfJsonSchemaPropertyInput, BuildError> {
        Ok(AnyOfJsonSchemaPropertyInput {
            any_of: self.any_of.ok_or_else(|| BuildError::missing_field("any_of"))?,
            discriminator: self.discriminator.ok_or_else(|| BuildError::missing_field("discriminator"))?,
            description: self.description,
        })
    }
}
