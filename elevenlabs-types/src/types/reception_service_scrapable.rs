pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Scrape-reachable subset of a service (classic, rental or group).
/// 
/// The extraction pipeline uses one LLM prompt per kind (so the model sees
/// kind-specific guidance), but every code path produces this single shape:
/// onboarding, imports and the unified ``smb_services`` collection all
/// discriminate by the ``kind`` field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionServiceScrapable {
    /// 'fixed' (default): the item has a known price the agent can quote, and customers can book or order it directly. 'quote': the price depends on the job (renovation, catering, custom work), so no price or slot is offered; the agent collects the intake_questions and files a quote request instead. Takes effect only while quotes are enabled for the business. Group services cannot use 'quote'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_mode: Option<PricingMode>,
    /// Group memberships for this item. Each entry carries the group label and the item's 0-based rank within that group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReceptionGroupMembership>>,
    /// Display position among items with no group; `null` sorts last. Change it with the reorder endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ungrouped_rank: Option<i64>,
    /// Service name shown to customers and the receptionist.
    #[serde(default)]
    pub name: StrippedStr,
    /// Details about the service for customers and the receptionist; empty string means none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<StrippedStr>,
    /// Bookable options for this service. Always non-empty; `variants[0]` is the default option shown when callers don't pick a specific variant. Each entry has its own duration and (optional) price, and uniqueness is keyed by duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<ReceptionServiceVariant>>,
    /// Classic = staff-delivered; rental = asset-based; group = multi-participant session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ReceptionServiceKind>,
    /// Turnover/cleanup time reserved after each booking. Applies to every service kind; default 0 means no buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_minutes: Option<i64>,
    /// Where the service is delivered. `business` = client visits the business location (default); `customer` = staff travels to the client. Only meaningful for `kind='classic'`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_location: Option<ServiceLocationType>,
    /// Optional extras that can be added to a booking of this service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<ReceptionAddOn>>,
    /// Capacity for `kind='group'`: how many seats a single session can sell across all registrations. Must be null for classic and rental services (those are always exclusive to one client per booking).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<i64>,
}

impl ReceptionServiceScrapable {
    pub fn builder() -> ReceptionServiceScrapableBuilder {
        <ReceptionServiceScrapableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionServiceScrapableBuilder {
    pricing_mode: Option<PricingMode>,
    groups: Option<Vec<ReceptionGroupMembership>>,
    ungrouped_rank: Option<i64>,
    name: Option<StrippedStr>,
    description: Option<StrippedStr>,
    variants: Option<Vec<ReceptionServiceVariant>>,
    kind: Option<ReceptionServiceKind>,
    buffer_minutes: Option<i64>,
    service_location: Option<ServiceLocationType>,
    add_ons: Option<Vec<ReceptionAddOn>>,
    max_participants: Option<i64>,
}

impl ReceptionServiceScrapableBuilder {
    pub fn pricing_mode(mut self, value: PricingMode) -> Self {
        self.pricing_mode = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<ReceptionGroupMembership>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn ungrouped_rank(mut self, value: i64) -> Self {
        self.ungrouped_rank = Some(value);
        self
    }

    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn description(mut self, value: StrippedStr) -> Self {
        self.description = Some(value);
        self
    }

    pub fn variants(mut self, value: Vec<ReceptionServiceVariant>) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn kind(mut self, value: ReceptionServiceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn buffer_minutes(mut self, value: i64) -> Self {
        self.buffer_minutes = Some(value);
        self
    }

    pub fn service_location(mut self, value: ServiceLocationType) -> Self {
        self.service_location = Some(value);
        self
    }

    pub fn add_ons(mut self, value: Vec<ReceptionAddOn>) -> Self {
        self.add_ons = Some(value);
        self
    }

    pub fn max_participants(mut self, value: i64) -> Self {
        self.max_participants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionServiceScrapable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionServiceScrapableBuilder::name)
    pub fn build(self) -> Result<ReceptionServiceScrapable, BuildError> {
        Ok(ReceptionServiceScrapable {
            pricing_mode: self.pricing_mode,
            groups: self.groups,
            ungrouped_rank: self.ungrouped_rank,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            variants: self.variants,
            kind: self.kind,
            buffer_minutes: self.buffer_minutes,
            service_location: self.service_location,
            add_ons: self.add_ons,
            max_participants: self.max_participants,
        })
    }
}
