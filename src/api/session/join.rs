use rocket::serde::json::Json;
use rocket_empty::EmptyResponse;

use crate::{Result, Ygg};

/// # Information to join a Minecraft server
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadJoinServer {
    /// Access token
    pub access_token: String,

    /// Player's UUID without the dashes
    pub selected_profile: String,

    /// Server hash
    pub server_id: String,
}

/// # Join a server
///
/// https://wiki.vg/Protocol_Encryption#Client
#[openapi(tag = "Minecraft Session Server")]
#[post("/minecraft/join", data = "<data>")]
pub async fn join(ygg: &Ygg, data: Json<PayloadJoinServer>) -> Result<EmptyResponse> {
    ygg.join_server(data.into_inner())
        .await
        .map(|_| EmptyResponse)
}
