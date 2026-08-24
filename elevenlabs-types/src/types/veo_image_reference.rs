pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A reference image guiding a Veo generation, with its role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VeoImageReference {
    /// The reference image.
    pub image: ImageReference,
    /// How the model uses the image: `subject` places its subject or scene elements into the video; `style` transfers its visual style.
    pub role: VeoImageReferenceRole,
}

impl VeoImageReference {
    pub fn builder() -> VeoImageReferenceBuilder {
        <VeoImageReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VeoImageReferenceBuilder {
    image: Option<ImageReference>,
    role: Option<VeoImageReferenceRole>,
}

impl VeoImageReferenceBuilder {
    pub fn image(mut self, value: ImageReference) -> Self {
        self.image = Some(value);
        self
    }

    pub fn role(mut self, value: VeoImageReferenceRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VeoImageReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image`](VeoImageReferenceBuilder::image)
    /// - [`role`](VeoImageReferenceBuilder::role)
    pub fn build(self) -> Result<VeoImageReference, BuildError> {
        Ok(VeoImageReference {
            image: self.image.ok_or_else(|| BuildError::missing_field("image"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
