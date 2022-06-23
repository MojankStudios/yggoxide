use rocket::serde::json::Json;

use crate::structs::yggdrasil::Information;

/// # Information
///
/// Information about the authentication server.
///
/// Dervied from https://authserver.mojang.com/
#[openapi(tag = "Yggdrasil Auth Server")]
#[get("/")]
pub async fn index() -> Json<Information> {
    Json(Default::default())
}
