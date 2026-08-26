pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    /// The unique identifier of the user.
    #[serde(default)]
    pub user_id: String,
    /// Details of the user's subscription.
    pub subscription: SubscriptionResponse,
    /// Whether the user is new. This field is deprecated and will be removed in the future. Use 'created_at' instead.
    #[serde(default)]
    pub is_new_user: bool,
    /// This field is deprecated and will be removed in a future major version. Instead use subscription.trust_on_invoice_creation.
    #[serde(default)]
    pub can_use_delayed_payment_methods: bool,
    /// Whether the user's onboarding is completed.
    #[serde(default)]
    pub is_onboarding_completed: bool,
    /// Whether the user's onboarding checklist is completed.
    #[serde(default)]
    pub is_onboarding_checklist_completed: bool,
    /// Whether to show compliance terms (ToS, Privacy Policy, biometric consent) during onboarding. Set for users signing up from the marketing site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_compliance_terms: Option<bool>,
    /// First name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Whether the user's API key is hashed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_api_key_hashed: Option<bool>,
    /// The preview of the user's API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key_preview: Option<String>,
    /// The referral link code of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_link_code: Option<String>,
    /// The Partnerstack partner default link of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partnerstack_partner_default_link: Option<String>,
    /// The unix timestamp of the user's creation. 0 if the user was created before the unix timestamp was added.
    #[serde(default)]
    pub created_at: i64,
    /// The seat type of the user.
    pub seat_type: SeatType,
}

impl User {
    pub fn builder() -> UserBuilder {
        <UserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBuilder {
    user_id: Option<String>,
    subscription: Option<SubscriptionResponse>,
    is_new_user: Option<bool>,
    can_use_delayed_payment_methods: Option<bool>,
    is_onboarding_completed: Option<bool>,
    is_onboarding_checklist_completed: Option<bool>,
    show_compliance_terms: Option<bool>,
    first_name: Option<String>,
    is_api_key_hashed: Option<bool>,
    xi_api_key_preview: Option<String>,
    referral_link_code: Option<String>,
    partnerstack_partner_default_link: Option<String>,
    created_at: Option<i64>,
    seat_type: Option<SeatType>,
}

impl UserBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn subscription(mut self, value: SubscriptionResponse) -> Self {
        self.subscription = Some(value);
        self
    }

    pub fn is_new_user(mut self, value: bool) -> Self {
        self.is_new_user = Some(value);
        self
    }

    pub fn can_use_delayed_payment_methods(mut self, value: bool) -> Self {
        self.can_use_delayed_payment_methods = Some(value);
        self
    }

    pub fn is_onboarding_completed(mut self, value: bool) -> Self {
        self.is_onboarding_completed = Some(value);
        self
    }

    pub fn is_onboarding_checklist_completed(mut self, value: bool) -> Self {
        self.is_onboarding_checklist_completed = Some(value);
        self
    }

    pub fn show_compliance_terms(mut self, value: bool) -> Self {
        self.show_compliance_terms = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn is_api_key_hashed(mut self, value: bool) -> Self {
        self.is_api_key_hashed = Some(value);
        self
    }

    pub fn xi_api_key_preview(mut self, value: impl Into<String>) -> Self {
        self.xi_api_key_preview = Some(value.into());
        self
    }

    pub fn referral_link_code(mut self, value: impl Into<String>) -> Self {
        self.referral_link_code = Some(value.into());
        self
    }

    pub fn partnerstack_partner_default_link(mut self, value: impl Into<String>) -> Self {
        self.partnerstack_partner_default_link = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn seat_type(mut self, value: SeatType) -> Self {
        self.seat_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`User`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](UserBuilder::user_id)
    /// - [`subscription`](UserBuilder::subscription)
    /// - [`is_new_user`](UserBuilder::is_new_user)
    /// - [`can_use_delayed_payment_methods`](UserBuilder::can_use_delayed_payment_methods)
    /// - [`is_onboarding_completed`](UserBuilder::is_onboarding_completed)
    /// - [`is_onboarding_checklist_completed`](UserBuilder::is_onboarding_checklist_completed)
    /// - [`created_at`](UserBuilder::created_at)
    /// - [`seat_type`](UserBuilder::seat_type)
    pub fn build(self) -> Result<User, BuildError> {
        Ok(User {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            subscription: self.subscription.ok_or_else(|| BuildError::missing_field("subscription"))?,
            is_new_user: self.is_new_user.ok_or_else(|| BuildError::missing_field("is_new_user"))?,
            can_use_delayed_payment_methods: self.can_use_delayed_payment_methods.ok_or_else(|| BuildError::missing_field("can_use_delayed_payment_methods"))?,
            is_onboarding_completed: self.is_onboarding_completed.ok_or_else(|| BuildError::missing_field("is_onboarding_completed"))?,
            is_onboarding_checklist_completed: self.is_onboarding_checklist_completed.ok_or_else(|| BuildError::missing_field("is_onboarding_checklist_completed"))?,
            show_compliance_terms: self.show_compliance_terms,
            first_name: self.first_name,
            is_api_key_hashed: self.is_api_key_hashed,
            xi_api_key_preview: self.xi_api_key_preview,
            referral_link_code: self.referral_link_code,
            partnerstack_partner_default_link: self.partnerstack_partner_default_link,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            seat_type: self.seat_type.ok_or_else(|| BuildError::missing_field("seat_type"))?,
        })
    }
}
