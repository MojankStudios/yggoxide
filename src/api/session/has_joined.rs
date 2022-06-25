use rocket::serde::json::Json;

use crate::{structs::session::Profile, Result, Ygg};

/// # Information about user who is joining
#[derive(FromForm, JsonSchema)]
pub struct QueryHasJoined {
    /// Username
    pub username: String,

    /// Server hash
    #[field(name = "serverId")]
    pub server_id: String,

    /// User IP address
    pub ip: Option<String>,
}

/// # Check if user has joined
///
/// https://wiki.vg/Protocol_Encryption#Server
#[openapi(tag = "Minecraft Session Server")]
#[get("/session/minecraft/hasJoined?<data..>")]
pub async fn has_joined(ygg: &Ygg, data: QueryHasJoined) -> Result<Json<Profile>> {
    ygg.has_joined(data).await.map(Json)
}
