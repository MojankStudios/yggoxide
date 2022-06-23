use rocket::serde::json::Json;

use crate::structs::session::Profile;

/// # Fetch a user's profile
#[openapi(tag = "Minecraft Session Server")]
#[get("/minecraft/profile/<uuid>")]
pub async fn fetch_profile(uuid: String) -> Json<Profile> {
    todo!()
}
