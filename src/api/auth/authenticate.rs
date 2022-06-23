use rocket::serde::json::Json;

use crate::structs::yggdrasil::{Agent, Profile, User};

/// Information to log into Yggdrasil
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadAuthenticate {
    /// Information about the client connecting
    agent: Agent,

    /// Account name (email or player name)
    username: String,

    /// Account password
    password: String,

    /// Optional client identifier
    client_token: Option<String>,

    /// Whether to add the `user` object to the response
    request_user: Option<bool>,
}

/// Response with access token for further requests
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAuthenticate {
    /// User
    ///
    /// Will only be returned if explicitly asked for
    user: Option<User>,

    /// Client identifier
    client_token: String,

    /// Hexadecimal or JWT
    ///
    /// The normal accessToken can be found in the payload of the JWT (second by '.' separated part as Base64 encoded JSON object), in key "yggt"
    access_token: String,

    /// Available profiles
    available_profiles: Vec<Profile>,

    /// Selected profile
    selected_profile: Profile,
}

/// # Authenticate
///
/// Authenticates a user using their password.
///
/// https://wiki.vg/Authentication#Authenticate
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/authenticate", data = "<data>")]
pub async fn authenticate(data: Json<PayloadAuthenticate>) -> Json<ResponseAuthenticate> {
    todo!()
}
