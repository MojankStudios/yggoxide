use rocket::serde::json::Json;
use rocket_empty::EmptyResponse;

use crate::{Result, Ygg};

/// # Information about known access token
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadValidate {
    /// Client identifier
    pub client_token: String,

    /// Hexadecimal or JWT
    pub access_token: String,
}

/// # Validate
///
/// Checks if an `accessToken` is usable for authentication with a Minecraft server. The Minecraft Launcher (as of version 1.6.13) calls this endpoint on startup to verify that its saved token is still usable, and calls `/refresh` if this returns an error.
///
/// https://wiki.vg/Authentication#Validate
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/validate", data = "<data>")]
pub async fn validate(ygg: &Ygg, data: Json<PayloadValidate>) -> Result<EmptyResponse> {
    ygg.validate(data.into_inner()).await.map(|_| EmptyResponse)
}
