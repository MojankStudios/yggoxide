use crate::{
    structs::services::{AccessToken, PlayerAttributes, PlayerCertificate},
    Result,
};

#[async_trait]
pub trait Services: Sync + Send {
    /// Fetch attributes for the currently authenticated player
    async fn fetch_attributes(&self, token: AccessToken) -> Result<PlayerAttributes>;

    /// Fetch key-pair for the currently authenticated player
    async fn fetch_certificate(&self, token: AccessToken) -> Result<PlayerCertificate>;
}
