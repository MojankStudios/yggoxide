/// # Invalidate
///
/// Invalidates `accessTokens` using a client/access token pair.
///
/// https://wiki.vg/Authentication#Invalidate
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/invalidate")]
pub async fn invalidate() {}
