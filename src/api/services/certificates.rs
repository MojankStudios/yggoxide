use rocket::serde::json::Json;

use crate::{
    structs::services::{AccessToken, PlayerCertificate},
    Result, Ygg,
};

/// # Fetch player's key-pair
///
/// Fetches the Mojang provided key-pair for a player, which are used for cryptographically signing chat messages.
#[openapi(tag = "Minecraft Services API")]
#[post("/player/certificates")]
pub async fn fetch_certificates(ygg: &Ygg, token: AccessToken) -> Result<Json<PlayerCertificate>> {
    ygg.fetch_certificate(token).await.map(Json)
}
