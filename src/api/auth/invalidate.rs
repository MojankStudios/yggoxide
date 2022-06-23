use rocket::serde::json::Json;
use rocket_empty::EmptyResponse;

use crate::{Result, Ygg};

/// # Information to log into Yggdrasil
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayloadInvalidate {
    /// Client identifier
    client_token: String,

    /// Access token
    access_token: String,
}

/// # Invalidate
///
/// Invalidates `accessTokens` using a client/access token pair.
///
/// https://wiki.vg/Authentication#Invalidate
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/invalidate", data = "<data>")]
pub async fn invalidate(ygg: &Ygg, data: Json<PayloadInvalidate>) -> Result<EmptyResponse> {
    ygg.invalidate(data.into_inner())
        .await
        .map(|_| EmptyResponse)
}
