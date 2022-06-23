use rocket::serde::json::Json;

use crate::{
    structs::yggdrasil::{AuthenticationProfile, User},
    Result, Ygg,
};

/// # Information to refresh access token
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadRefresh {
    /// Client identifier
    client_token: String,

    /// Access token
    access_token: String,

    /// Selected profile
    selected_profile: AuthenticationProfile,

    /// Whether to add the `user` object to the response
    request_user: Option<bool>,
}

/// # Response with new access token
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRefresh {
    /// User
    ///
    /// Will only be returned if explicitly asked for
    user: Option<User>,

    /// Client identifier
    client_token: String,

    /// Access token
    access_token: String,

    /// Selected profile
    selected_profile: AuthenticationProfile,
}

/// # Refresh
///
/// Refreshes a valid `accessToken`. It can be used to keep a user logged in between gaming sessions and is preferred over storing the user's password in a file.
///
/// https://wiki.vg/Authentication#Refresh
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/refresh", data = "<data>")]
pub async fn refresh(ygg: &Ygg, data: Json<PayloadRefresh>) -> Result<Json<ResponseRefresh>> {
    ygg.refresh(data.into_inner()).await.map(Json)
}
