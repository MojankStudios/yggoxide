use rocket::serde::json::Json;

use crate::{structs::session::Profile, Result, Ygg};

/// # Information about user who is joining
#[derive(FromForm, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryHasJoined {
    /// Username
    username: String,

    /// Server hash
    server_id: String,

    /// User IP address
    ip: String,
}

/// # Check if user has joined
///
/// https://wiki.vg/Protocol_Encryption#Server
#[openapi(tag = "Minecraft Session Server")]
#[get("/minecraft/hasJoined?<data..>")]
pub async fn has_joined(ygg: &Ygg, data: QueryHasJoined) -> Result<Json<Profile>> {
    ygg.has_joined(data).await.map(Json)
}
