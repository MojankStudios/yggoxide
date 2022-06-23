use rocket::serde::json::Json;
use rocket_empty::EmptyResponse;

use crate::Result;

/// # Information to join a Minecraft server
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadJoinServer {
    /// Access token
    access_token: String,

    /// Player's UUID without the dashes
    selected_profile: String,

    /// Server hash
    server_id: String,
}

/// # Join a server
///
/// https://wiki.vg/Protocol_Encryption#Client
#[openapi(tag = "Minecraft Session Server")]
#[post("/minecraft/join", data = "<data>")]
pub async fn join(data: Json<PayloadJoinServer>) -> Result<EmptyResponse> {
    todo!()
}
