use rocket::serde::json::Json;

use crate::{
    structs::services::{AccessToken, PlayerAttributes},
    Result, Ygg,
};

/// # Fetch player safety / ban attributes
///
/// Fetch information about players' online safety settings and ban status.
#[openapi(tag = "Minecraft Services API")]
#[get("/player/attributes")]
pub async fn fetch_attributes(ygg: &Ygg, token: AccessToken) -> Result<Json<PlayerAttributes>> {
    todo!()
}
