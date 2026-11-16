use crate::{ApiError, ClientConfig, HttpClient};

pub mod members;
pub use members::MembersClient;
pub mod invites;
pub use invites::InvitesClient;
pub struct TeamClient {
    pub http_client: HttpClient,
    pub members: MembersClient,
    pub invites: InvitesClient,
}

impl TeamClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            members: MembersClient::new(config.clone())?,
            invites: InvitesClient::new(config.clone())?,
        })
    }
}
