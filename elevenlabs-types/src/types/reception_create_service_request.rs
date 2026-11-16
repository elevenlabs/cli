pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionCreateServiceRequest {
    /// 'fixed' (default): the item has a known price the agent can quote, and customers can book or order it directly. 'quote': the price depends on the job (renovation, catering, custom work), so no price or slot is offered; the agent collects the intake_questions and files a quote request instead. Takes effect only while quotes are enabled for the business. Group services cannot use 'quote'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_mode: Option<PricingMode>,
    /// Questions the agent asks before booking or ordering this item. Required for a useful `pricing_mode='quote'` item -- without them a quote request arrives with nothing to price -- but also valid on fixed-price items that need extra detail (current hair color before a dye job, allergies before a treatment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_questions: Option<Vec<ReceptionIntakeQuestion>>,
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
    /// Not used to limit bookings; set `max_participants` on group services instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// Whether the service is offered; inactive services can't be booked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Whether any staff member can deliver the service. When false, only the staff in `staff_ids` can. Ignored for rentals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_staff_member: Option<bool>,
    /// Staff assigned to this service; must be empty for kind='rental'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff_ids: Option<Vec<String>>,
    /// Assets, such as rooms or equipment, this service can be booked on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<String>>,
    /// How far into the future customer-facing channels (public booking page, voice agent) may book this service. Applies to every kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_booking_minutes: Option<i64>,
    /// Locations where this service is offered; empty list means all locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    /// Whether the service is listed on the public booking page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_on_booking_page: Option<bool>,
    /// Travel time in minutes added before/after appointments for customer-location services. Blocks the staff member's calendar but is not visible to the customer. Ignored when `service_location='business'`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_minutes: Option<i64>,
    /// Add travel time before the appointment (outbound trip).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_before: Option<bool>,
    /// Add travel time after the appointment (return trip).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_after: Option<bool>,
    /// Time-bound discounts applicable to this service or specific variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Vec<ReceptionServicePromotion>>,
}

impl ReceptionCreateServiceRequest {
    pub fn builder() -> ReceptionCreateServiceRequestBuilder {
        <ReceptionCreateServiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCreateServiceRequestBuilder {
    pricing_mode: Option<PricingMode>,
    intake_questions: Option<Vec<ReceptionIntakeQuestion>>,
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
    capacity: Option<i64>,
    is_active: Option<bool>,
    any_staff_member: Option<bool>,
    staff_ids: Option<Vec<String>>,
    asset_ids: Option<Vec<String>>,
    advance_booking_minutes: Option<i64>,
    location_ids: Option<Vec<String>>,
    display_on_booking_page: Option<bool>,
    travel_minutes: Option<i64>,
    travel_before: Option<bool>,
    travel_after: Option<bool>,
    promotions: Option<Vec<ReceptionServicePromotion>>,
}

impl ReceptionCreateServiceRequestBuilder {
    pub fn pricing_mode(mut self, value: PricingMode) -> Self {
        self.pricing_mode = Some(value);
        self
    }

    pub fn intake_questions(mut self, value: Vec<ReceptionIntakeQuestion>) -> Self {
        self.intake_questions = Some(value);
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

    pub fn capacity(mut self, value: i64) -> Self {
        self.capacity = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn any_staff_member(mut self, value: bool) -> Self {
        self.any_staff_member = Some(value);
        self
    }

    pub fn staff_ids(mut self, value: Vec<String>) -> Self {
        self.staff_ids = Some(value);
        self
    }

    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn advance_booking_minutes(mut self, value: i64) -> Self {
        self.advance_booking_minutes = Some(value);
        self
    }

    pub fn location_ids(mut self, value: Vec<String>) -> Self {
        self.location_ids = Some(value);
        self
    }

    pub fn display_on_booking_page(mut self, value: bool) -> Self {
        self.display_on_booking_page = Some(value);
        self
    }

    pub fn travel_minutes(mut self, value: i64) -> Self {
        self.travel_minutes = Some(value);
        self
    }

    pub fn travel_before(mut self, value: bool) -> Self {
        self.travel_before = Some(value);
        self
    }

    pub fn travel_after(mut self, value: bool) -> Self {
        self.travel_after = Some(value);
        self
    }

    pub fn promotions(mut self, value: Vec<ReceptionServicePromotion>) -> Self {
        self.promotions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionCreateServiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionCreateServiceRequestBuilder::name)
    pub fn build(self) -> Result<ReceptionCreateServiceRequest, BuildError> {
        Ok(ReceptionCreateServiceRequest {
            pricing_mode: self.pricing_mode,
            intake_questions: self.intake_questions,
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
            capacity: self.capacity,
            is_active: self.is_active,
            any_staff_member: self.any_staff_member,
            staff_ids: self.staff_ids,
            asset_ids: self.asset_ids,
            advance_booking_minutes: self.advance_booking_minutes,
            location_ids: self.location_ids,
            display_on_booking_page: self.display_on_booking_page,
            travel_minutes: self.travel_minutes,
            travel_before: self.travel_before,
            travel_after: self.travel_after,
            promotions: self.promotions,
        })
    }
}

