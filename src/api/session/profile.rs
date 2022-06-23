use rocket::serde::json::Json;

use crate::{structs::session::Profile, Result};

/// # Fetch a user's profile
#[openapi(tag = "Minecraft Session Server")]
#[get("/minecraft/profile/<uuid>")]
pub async fn fetch_profile(uuid: String) -> Result<Json<Profile>> {
    todo!()
}
