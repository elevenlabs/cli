pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptionStyleSectionAnimationModel {
    pub enter_type: CaptionStyleSectionAnimationModelEnterType,
    pub exit_type: CaptionStyleSectionAnimationModelExitType,
}

impl CaptionStyleSectionAnimationModel {
    pub fn builder() -> CaptionStyleSectionAnimationModelBuilder {
        <CaptionStyleSectionAnimationModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleSectionAnimationModelBuilder {
    enter_type: Option<CaptionStyleSectionAnimationModelEnterType>,
    exit_type: Option<CaptionStyleSectionAnimationModelExitType>,
}

impl CaptionStyleSectionAnimationModelBuilder {
    pub fn enter_type(mut self, value: CaptionStyleSectionAnimationModelEnterType) -> Self {
        self.enter_type = Some(value);
        self
    }

    pub fn exit_type(mut self, value: CaptionStyleSectionAnimationModelExitType) -> Self {
        self.exit_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleSectionAnimationModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enter_type`](CaptionStyleSectionAnimationModelBuilder::enter_type)
    /// - [`exit_type`](CaptionStyleSectionAnimationModelBuilder::exit_type)
    pub fn build(self) -> Result<CaptionStyleSectionAnimationModel, BuildError> {
        Ok(CaptionStyleSectionAnimationModel {
            enter_type: self.enter_type.ok_or_else(|| BuildError::missing_field("enter_type"))?,
            exit_type: self.exit_type.ok_or_else(|| BuildError::missing_field("exit_type"))?,
        })
    }
}
