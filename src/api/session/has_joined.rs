/// # Check if user has joined
///
/// https://wiki.vg/Protocol_Encryption#Client
#[openapi(tag = "Minecraft Session Server")]
#[get("/minecraft/hasJoined")]
pub async fn has_joined() {}
