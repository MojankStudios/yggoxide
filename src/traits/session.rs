use crate::{
    api::session::{has_joined::QueryHasJoined, join::PayloadJoinServer},
    structs::session::Profile,
    Result,
};

#[async_trait]
pub trait Session: Sync + Send {
    /// Join a Minecraft server
    async fn join_server(&self, data: PayloadJoinServer) -> Result<()>;

    /// Check whether a client has joined a server and return their user profile
    async fn has_joined(&self, data: QueryHasJoined) -> Result<Profile>;

    /// Fetch a user's profile by their UUID
    async fn get_profile(&self, player_uuid: String) -> Result<Profile>;
}
