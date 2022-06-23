use rocket::serde::json::Json;
use rocket_empty::EmptyResponse;

/// Information to log into Yggdrasil
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadSignout {
    /// Account name (email or player name)
    username: String,

    /// Account password
    password: String,
}

/// # Signout
///
/// Invalidates `accessTokens` using an account's username and password.
///
/// https://wiki.vg/Authentication#Signout
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/signout", data = "<data>")]
pub async fn signout(data: Json<PayloadSignout>) -> EmptyResponse {
    todo!()
}
