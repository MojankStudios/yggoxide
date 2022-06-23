/// # Validate
///
/// Checks if an `accessToken` is usable for authentication with a Minecraft server. The Minecraft Launcher (as of version 1.6.13) calls this endpoint on startup to verify that its saved token is still usable, and calls `/refresh` if this returns an error.
///
/// https://wiki.vg/Authentication#Validate
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/validate")]
pub async fn validate() {}
