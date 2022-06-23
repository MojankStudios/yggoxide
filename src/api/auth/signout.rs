use rocket::serde::json::Json;
use rocket_empty::EmptyResponse;

use crate::{Result, Ygg};

/// # User credentials to log out of
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadSignout {
    /// Account name (email or player name)
    pub username: String,

    /// Account password
    pub password: String,
}

/// # Signout
///
/// Invalidates `accessTokens` using an account's username and password.
///
/// https://wiki.vg/Authentication#Signout
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/signout", data = "<data>")]
pub async fn signout(ygg: &Ygg, data: Json<PayloadSignout>) -> Result<EmptyResponse> {
    ygg.signout(data.into_inner()).await.map(|_| EmptyResponse)
}
