use dashmap::DashMap;
use uuid::Uuid;
use yggoxide::{prelude::*, traits::services::Services};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate nanoid;

/// Spoof implementation which mirrors real accounts
///
/// Some restrictions to keep things simple:
/// - Any password is accepted during authentication.
/// - New access token created for each login, no way to revoke.
/// - Client tokens are ignored entirely.
#[derive(Default)]
pub struct SpoofImpl {
    /// Map of access tokens to user UUIDs
    auth: DashMap<String, Uuid>,

    /// Map of user UUIDs to user profiles
    user: DashMap<Uuid, Profile>,

    /// Map of usernames to user UUIDs
    uuids: DashMap<String, Uuid>,
}

/// Short hand for parsing UUIDs
fn puuid(v: &str) -> Result<Uuid> {
    Uuid::parse_str(v).map_err(|_| Error::IllegalArgumentException(Default::default()))
}

impl SpoofImpl {
    /// Generate a fake player
    fn fake_player(uuid: String, username: String) -> Profile {
        Profile {
            id: uuid,
            name: username,
            properties: vec![
                PlayerProperty::Textures {
                    value: "ewogICJ0aW1lc3RhbXAiIDogMTY1NjAxNzg1Mzg3OCwKICAicHJvZmlsZUlkIiA6ICI5ZmE4MDczOTQyOWM0Njg5OGY0N2ViMGRmYzhlNTI1OSIsCiAgInByb2ZpbGVOYW1lIiA6ICJDeW5vc3BoZXJlIiwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzE4MTVmZWRhODJjMjNiN2M3MjdmNjg2MWM1ZmQzYzUxYWIwYjI3ZGIzZmJkMTRjZmM5YzI3ZTFhODdmNzBlMDMiLAogICAgICAibWV0YWRhdGEiIDogewogICAgICAgICJtb2RlbCIgOiAic2xpbSIKICAgICAgfQogICAgfQogIH0KfQ==".to_string(),
                    signature: None
                }
            ]
        }
    }

    /// Resolve a user profile from a UUID and save information for future use
    ///
    /// If an empty string is given, a fake player is generated.
    async fn resolve_uuid<U>(&self, uuid: Option<&Uuid>, username_hint: U) -> Result<Profile>
    where
        U: Into<Option<String>>,
    {
        // Check if we already have a profile for this UUID
        if let Some(uuid) = uuid {
            if let Some(profile) = self.user.get(uuid) {
                return Ok(profile.clone());
            }
        }

        let profile = if let Some(uuid) = uuid {
            // Fetch player from Mojang's API
            if let Ok(res) = reqwest::get(format!(
                "https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"
            ))
            .await
            {
                res.json()
                    .await
                    .expect("Failed to deserialise data from Mojang")
            } else {
                SpoofImpl::fake_player(
                    uuid.to_string(),
                    username_hint
                        .into()
                        .unwrap_or_else(|| "NoPlayer".to_string()),
                )
            }
        } else {
            // Create a fake player if no UUID
            let uuid = Uuid::new_v4();
            SpoofImpl::fake_player(
                uuid.to_string(),
                username_hint
                    .into()
                    .unwrap_or_else(|| "NoUsername".to_string()),
            )
        };

        self.uuids
            .insert(profile.name.to_string(), puuid(&profile.id)?);

        self.user.insert(puuid(&profile.id)?, profile.clone());

        Ok(profile)
    }

    /// Resolve a user profile from a username and save information for future use
    async fn resolve_username(&self, username: &str) -> Result<Profile> {
        // Check if we already have info about this username
        if let Some(uuid) = self.uuids.get(username) {
            return self.resolve_uuid(Some(&uuid), None).await;
        }

        // Otherwise resolve the username from Mojang's API
        let res = reqwest::get(format!(
            "https://api.mojang.com/users/profiles/minecraft/{username}"
        ))
        .await
        .expect("Failed to contact Mojang's API");

        // Take the UUID if existing otherwise leave empty
        let uuid = if let reqwest::StatusCode::NO_CONTENT = res.status() {
            Default::default()
        } else {
            let profile: AuthenticationProfile = res
                .json()
                .await
                .expect("Failed to deserialise profile from Mojang");

            puuid(&profile.id)?
        };

        self.resolve_uuid(Some(&uuid), username.to_string()).await
    }
}

#[async_trait]
impl Auth for SpoofImpl {
    /// Authenticate a user with the service
    async fn authenticate(&self, data: PayloadAuthenticate) -> Result<ResponseAuthenticate> {
        let profile = self.resolve_username(&data.username).await?;
        let access_token = nanoid!();

        self.auth
            .insert(access_token.to_string(), puuid(&profile.id)?);

        Ok(ResponseAuthenticate {
            access_token,
            client_token: data.client_token.unwrap_or_default(),
            available_profiles: vec![profile.clone().into()],
            selected_profile: profile.clone().into(),
            user: Some(profile.into()),
        })
    }

    /// Refresh an access token
    async fn refresh(&self, data: PayloadRefresh) -> Result<ResponseRefresh> {
        let selected_profile = data.selected_profile;
        let profile = self
            .resolve_uuid(Some(&puuid(&selected_profile.id)?), None)
            .await?;

        Ok(ResponseRefresh {
            access_token: data.access_token,
            client_token: data.client_token,
            selected_profile,
            user: Some(profile.into()),
        })
    }

    /// Validate an access token
    async fn validate(&self, data: PayloadValidate) -> Result<()> {
        // Check if this user is known.
        if !self.auth.contains_key(&data.access_token) {
            return Err(Error::ForbiddenOperationException(Default::default()));
        }

        Ok(())
    }

    /// Sign out using credentials
    async fn signout(&self, _data: PayloadSignout) -> Result<()> {
        // Ignore attempts to sign out.
        Ok(())
    }

    /// Invalidate current access token
    async fn invalidate(&self, _data: PayloadInvalidate) -> Result<()> {
        // Ignore attempts to invalidate.
        Ok(())
    }
}

#[async_trait]
impl Session for SpoofImpl {
    /// Join a Minecraft server
    async fn join_server(&self, data: PayloadJoinServer) -> Result<()> {
        // Just make sure we have the required data.
        if !self.auth.contains_key(&data.access_token) {
            return Err(Error::ForbiddenOperationException(Default::default()));
        }

        // Always allow joining any server.
        Ok(())
    }

    /// Check whether a client has joined a server and return their user profile
    async fn has_joined(&self, data: QueryHasJoined) -> Result<Profile> {
        Ok(self.resolve_username(&data.username).await?)
    }

    /// Fetch a user's profile by their UUID
    async fn get_profile(&self, player_uuid: String) -> Result<Profile> {
        Ok(self.resolve_uuid(Some(&puuid(&player_uuid)?), None).await?)
    }
}

#[async_trait]
impl Services for SpoofImpl {
    /// Fetch attributes for the currently authenticated player
    async fn fetch_attributes(&self, _token: AccessToken) -> Result<PlayerAttributes> {
        Ok(Default::default())
    }

    /// Fetch key-pair for the currently authenticated player
    async fn fetch_certificate(&self, _token: AccessToken) -> Result<PlayerCertificate> {
        Ok(Default::default())
    }
}

#[launch]
fn rocket() -> _ {
    build_managed(Box::new(SpoofImpl::default()))
}
