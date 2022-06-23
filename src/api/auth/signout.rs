/// # Signout
///
/// Invalidates `accessTokens` using an account's username and password.
///
/// https://wiki.vg/Authentication#Signout
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/signout")]
pub async fn signout() {}
