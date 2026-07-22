pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptionStyleCharacterAnimationModel {
    pub enter_type: CaptionStyleCharacterAnimationModelEnterType,
    pub exit_type: CaptionStyleCharacterAnimationModelExitType,
}

impl CaptionStyleCharacterAnimationModel {
    pub fn builder() -> CaptionStyleCharacterAnimationModelBuilder {
        <CaptionStyleCharacterAnimationModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleCharacterAnimationModelBuilder {
    enter_type: Option<CaptionStyleCharacterAnimationModelEnterType>,
    exit_type: Option<CaptionStyleCharacterAnimationModelExitType>,
}

impl CaptionStyleCharacterAnimationModelBuilder {
    pub fn enter_type(mut self, value: CaptionStyleCharacterAnimationModelEnterType) -> Self {
        self.enter_type = Some(value);
        self
    }

    pub fn exit_type(mut self, value: CaptionStyleCharacterAnimationModelExitType) -> Self {
        self.exit_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleCharacterAnimationModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enter_type`](CaptionStyleCharacterAnimationModelBuilder::enter_type)
    /// - [`exit_type`](CaptionStyleCharacterAnimationModelBuilder::exit_type)
    pub fn build(self) -> Result<CaptionStyleCharacterAnimationModel, BuildError> {
        Ok(CaptionStyleCharacterAnimationModel {
            enter_type: self.enter_type.ok_or_else(|| BuildError::missing_field("enter_type"))?,
            exit_type: self.exit_type.ok_or_else(|| BuildError::missing_field("exit_type"))?,
        })
    }
}
