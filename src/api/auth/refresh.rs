/// # Refresh
///
/// Refreshes a valid `accessToken`. It can be used to keep a user logged in between gaming sessions and is preferred over storing the user's password in a file.
///
/// https://wiki.vg/Authentication#Refresh
#[openapi(tag = "Yggdrasil Auth Server")]
#[post("/refresh")]
pub async fn refresh() {}
