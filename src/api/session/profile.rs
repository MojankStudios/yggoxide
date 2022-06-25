use rocket::serde::json::Json;

use crate::{structs::session::Profile, Result, Ygg};

/// # Fetch a user's profile
#[openapi(tag = "Minecraft Session Server")]
#[get("/session/minecraft/profile/<uuid>")]
pub async fn fetch_profile(ygg: &Ygg, uuid: String) -> Result<Json<Profile>> {
    ygg.get_profile(uuid).await.map(Json)
}
