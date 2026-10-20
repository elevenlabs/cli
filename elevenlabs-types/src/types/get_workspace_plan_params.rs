pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Read the workspace's current Reception plan and its hard limits.
/// 
/// Call this before answering any plan, pricing, trial, or limit question so
/// the assistant stops guessing (e.g. telling a paying owner they're on the
/// trial, or hallucinating concurrent-call / receptionist / voice-cloning
/// caps). The tool exposes only the shape of the plan — a display name;
/// whether it's trial / paid / untiered; the entitlement caps (max
/// receptionists, phone numbers, active locations, team members, concurrent
/// calls, voice-cloning quota) alongside the current counts; and whether
/// there's a payment issue. It deliberately does NOT return list prices or
/// dollar amounts, nor the trial / renewal end date: pricing and renewal
/// dates are a product decision and live in the Settings > Billing plan
/// picker.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkspacePlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetWorkspacePlanParams {
    pub fn builder() -> GetWorkspacePlanParamsBuilder {
        <GetWorkspacePlanParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkspacePlanParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetWorkspacePlanParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWorkspacePlanParams`].
    pub fn build(self) -> Result<GetWorkspacePlanParams, BuildError> {
        Ok(GetWorkspacePlanParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
