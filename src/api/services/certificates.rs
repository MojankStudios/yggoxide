use rocket::serde::json::Json;

use crate::{
    structs::{services::AccessToken, session::Profile},
    Result, Ygg,
};

/// # Fetch player's key-pair
///
/// Fetches the Mojang provided key-pair for a player, which are used for cryptographically signing chat messages.
#[openapi(tag = "Minecraft Services API")]
#[get("/player/certificates")]
pub async fn fetch_certificates(ygg: &Ygg, token: AccessToken) {
    todo!()
}
