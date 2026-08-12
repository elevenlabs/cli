pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimGroupResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scim_external_id: Option<String>,
    #[serde(default)]
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<SeatType>,
}

impl ScimGroupResponseModel {
    pub fn builder() -> ScimGroupResponseModelBuilder {
        <ScimGroupResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimGroupResponseModelBuilder {
    scim_external_id: Option<String>,
    display_name: Option<String>,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    seat_type: Option<SeatType>,
}

impl ScimGroupResponseModelBuilder {
    pub fn scim_external_id(mut self, value: impl Into<String>) -> Self {
        self.scim_external_id = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: i64) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    pub fn seat_type(mut self, value: SeatType) -> Self {
        self.seat_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimGroupResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](ScimGroupResponseModelBuilder::display_name)
    pub fn build(self) -> Result<ScimGroupResponseModel, BuildError> {
        Ok(ScimGroupResponseModel {
            scim_external_id: self.scim_external_id,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix,
            seat_type: self.seat_type,
        })
    }
}
