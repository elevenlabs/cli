use crate::{ApiError, ClientConfig, HttpClient};

pub mod services;
pub use services::ServicesClient;
pub mod assets;
pub use assets::AssetsClient2;
pub mod products;
pub use products::ProductsClient;
pub mod staff;
pub use staff::StaffClient;
pub mod receptionists;
pub use receptionists::ReceptionistsClient;
pub mod phone_numbers;
pub use phone_numbers::PhoneNumbersClient2;
pub mod holidays;
pub use holidays::HolidaysClient;
pub mod locations;
pub use locations::LocationsClient;
pub mod business_profile;
pub use business_profile::BusinessProfileClient;
pub mod knowledge_sources;
pub use knowledge_sources::KnowledgeSourcesClient;
pub mod workspaces;
pub use workspaces::WorkspacesClient;
pub mod subscriptions;
pub use subscriptions::SubscriptionsClient;
pub mod tiers;
pub use tiers::TiersClient;
pub mod billing;
pub use billing::BillingClient;
pub mod team;
pub use team::TeamClient;
pub struct ReceptionClient {
    pub http_client: HttpClient,
    pub services: ServicesClient,
    pub assets: AssetsClient2,
    pub products: ProductsClient,
    pub staff: StaffClient,
    pub receptionists: ReceptionistsClient,
    pub phone_numbers: PhoneNumbersClient2,
    pub holidays: HolidaysClient,
    pub locations: LocationsClient,
    pub business_profile: BusinessProfileClient,
    pub knowledge_sources: KnowledgeSourcesClient,
    pub workspaces: WorkspacesClient,
    pub subscriptions: SubscriptionsClient,
    pub tiers: TiersClient,
    pub billing: BillingClient,
    pub team: TeamClient,
}

impl ReceptionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            services: ServicesClient::new(config.clone())?,
            assets: AssetsClient2::new(config.clone())?,
            products: ProductsClient::new(config.clone())?,
            staff: StaffClient::new(config.clone())?,
            receptionists: ReceptionistsClient::new(config.clone())?,
            phone_numbers: PhoneNumbersClient2::new(config.clone())?,
            holidays: HolidaysClient::new(config.clone())?,
            locations: LocationsClient::new(config.clone())?,
            business_profile: BusinessProfileClient::new(config.clone())?,
            knowledge_sources: KnowledgeSourcesClient::new(config.clone())?,
            workspaces: WorkspacesClient::new(config.clone())?,
            subscriptions: SubscriptionsClient::new(config.clone())?,
            tiers: TiersClient::new(config.clone())?,
            billing: BillingClient::new(config.clone())?,
            team: TeamClient::new(config.clone())?,
        })
    }
}
