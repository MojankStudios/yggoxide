use yggoxide::{prelude::*, structs::services::PlayerAttributes, traits::services::Services};

#[macro_use]
extern crate rocket;

pub struct TodoImpl;

#[async_trait]
impl Auth for TodoImpl {
    /// Authenticate a user with the service
    async fn authenticate(&self, _data: PayloadAuthenticate) -> Result<ResponseAuthenticate> {
        todo!()
    }

    /// Refresh an access token
    async fn refresh(&self, _data: PayloadRefresh) -> Result<ResponseRefresh> {
        todo!()
    }

    /// Validate an access token
    async fn validate(&self, _data: PayloadValidate) -> Result<()> {
        todo!()
    }

    /// Sign out using credentials
    async fn signout(&self, _data: PayloadSignout) -> Result<()> {
        todo!()
    }

    /// Invalidate current access token
    async fn invalidate(&self, _data: PayloadInvalidate) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Session for TodoImpl {
    /// Join a Minecraft server
    async fn join_server(&self, _data: PayloadJoinServer) -> Result<()> {
        todo!()
    }

    /// Check whether a client has joined a server and return their user profile
    async fn has_joined(&self, _data: QueryHasJoined) -> Result<Profile> {
        todo!()
    }

    /// Fetch a user's profile by their UUID
    async fn get_profile(&self, _player_uuid: Uuid) -> Result<Profile> {
        todo!()
    }
}

#[async_trait]
impl Services for TodoImpl {
    /// Fetch attributes for the currently authenticated player
    async fn fetch_attributes(&self, _token: AccessToken) -> Result<PlayerAttributes> {
        todo!()
    }

    /// Fetch key-pair for the currently authenticated player
    async fn fetch_certificate(&self, _token: AccessToken) -> Result<PlayerCertificate> {
        todo!()
    }
}

#[launch]
fn rocket() -> _ {
    build_managed(Box::new(TodoImpl))
}
