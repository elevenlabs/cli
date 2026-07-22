pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateOrderV1ProductionsOrdersOrderIdPatch {
    #[serde(default)]
    pub request: UpdateOrderRequest,
}

impl BodyUpdateOrderV1ProductionsOrdersOrderIdPatch {
    pub fn builder() -> BodyUpdateOrderV1ProductionsOrdersOrderIdPatchBuilder {
        <BodyUpdateOrderV1ProductionsOrdersOrderIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateOrderV1ProductionsOrdersOrderIdPatchBuilder {
    request: Option<UpdateOrderRequest>,
}

impl BodyUpdateOrderV1ProductionsOrdersOrderIdPatchBuilder {
    pub fn request(mut self, value: UpdateOrderRequest) -> Self {
        self.request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateOrderV1ProductionsOrdersOrderIdPatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request`](BodyUpdateOrderV1ProductionsOrdersOrderIdPatchBuilder::request)
    pub fn build(self) -> Result<BodyUpdateOrderV1ProductionsOrdersOrderIdPatch, BuildError> {
        Ok(BodyUpdateOrderV1ProductionsOrdersOrderIdPatch {
            request: self.request.ok_or_else(|| BuildError::missing_field("request"))?,
        })
    }
}

