pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionServicesListQueryRequest {
    /// Only services this staff member performs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff_id: Option<String>,
    /// Only services that book this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Only active (`true`) or inactive (`false`) services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Only services of this kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ReceptionServiceKind>,
}

impl ReceptionServicesListQueryRequest {
    pub fn builder() -> ReceptionServicesListQueryRequestBuilder {
        <ReceptionServicesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionServicesListQueryRequestBuilder {
    staff_id: Option<String>,
    asset_id: Option<String>,
    is_active: Option<bool>,
    kind: Option<ReceptionServiceKind>,
}

impl ReceptionServicesListQueryRequestBuilder {
    pub fn staff_id(mut self, value: impl Into<String>) -> Self {
        self.staff_id = Some(value.into());
        self
    }

    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn kind(mut self, value: ReceptionServiceKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionServicesListQueryRequest`].
    pub fn build(self) -> Result<ReceptionServicesListQueryRequest, BuildError> {
        Ok(ReceptionServicesListQueryRequest {
            staff_id: self.staff_id,
            asset_id: self.asset_id,
            is_active: self.is_active,
            kind: self.kind,
        })
    }
}

