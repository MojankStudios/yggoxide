use yggoxide::prelude::*;

#[macro_use]
extern crate rocket;

pub struct ExampleImpl;

#[async_trait]
impl Auth for ExampleImpl {
    /// Authenticate a user with the service
    async fn authenticate(&self, data: PayloadAuthenticate) -> Result<ResponseAuthenticate> {
        let id: Uuid = Default::default();

        Ok(ResponseAuthenticate {
            access_token: "a".to_string(),
            client_token: "a".to_string(),
            available_profiles: vec![AuthenticationProfile {
                id: id.clone(),
                name: data.username.clone(),
            }],
            selected_profile: AuthenticationProfile {
                id: id.clone(),
                name: data.username.clone(),
            },
            user: Some(User {
                id,
                username: data.username,
                properties: vec![],
            }),
        })
    }

    /// Refresh an access token
    async fn refresh(&self, data: PayloadRefresh) -> Result<ResponseRefresh> {
        Ok(ResponseRefresh {
            access_token: data.access_token,
            client_token: data.client_token,
            selected_profile: data.selected_profile,
            user: Some(User {
                id: Default::default(),
                username: "username".to_string(),
                properties: vec![],
            }),
        })
    }

    /// Validate an access token
    async fn validate(&self, _data: PayloadValidate) -> Result<()> {
        Ok(())
    }

    /// Sign out using credentials
    async fn signout(&self, _data: PayloadSignout) -> Result<()> {
        Ok(())
    }

    /// Invalidate current access token
    async fn invalidate(&self, _data: PayloadInvalidate) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl Session for ExampleImpl {
    /// Join a Minecraft server
    async fn join_server(&self, _data: PayloadJoinServer) -> Result<()> {
        Ok(())
    }

    /// Check whether a client has joined a server and return their user profile
    async fn has_joined(&self, data: QueryHasJoined) -> Result<Profile> {
        Ok(Profile {
            id: Default::default(),
            name: data.username,
            properties: vec![
                PlayerProperty::Textures {
                    value: "ewogICJ0aW1lc3RhbXAiIDogMTY1NjAxNzg1Mzg3OCwKICAicHJvZmlsZUlkIiA6ICI5ZmE4MDczOTQyOWM0Njg5OGY0N2ViMGRmYzhlNTI1OSIsCiAgInByb2ZpbGVOYW1lIiA6ICJDeW5vc3BoZXJlIiwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzE4MTVmZWRhODJjMjNiN2M3MjdmNjg2MWM1ZmQzYzUxYWIwYjI3ZGIzZmJkMTRjZmM5YzI3ZTFhODdmNzBlMDMiLAogICAgICAibWV0YWRhdGEiIDogewogICAgICAgICJtb2RlbCIgOiAic2xpbSIKICAgICAgfQogICAgfQogIH0KfQ==".to_string(),
                    signature: None
                }
            ]
        })
    }

    /// Fetch a user's profile by their UUID
    async fn get_profile(&self, player_uuid: Uuid) -> Result<Profile> {
        Ok(Profile {
            id: player_uuid,
            name: "username".to_string(),
            properties: vec![
                PlayerProperty::Textures {
                    value: "ewogICJ0aW1lc3RhbXAiIDogMTY1NjAxNzg1Mzg3OCwKICAicHJvZmlsZUlkIiA6ICI5ZmE4MDczOTQyOWM0Njg5OGY0N2ViMGRmYzhlNTI1OSIsCiAgInByb2ZpbGVOYW1lIiA6ICJDeW5vc3BoZXJlIiwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlLzE4MTVmZWRhODJjMjNiN2M3MjdmNjg2MWM1ZmQzYzUxYWIwYjI3ZGIzZmJkMTRjZmM5YzI3ZTFhODdmNzBlMDMiLAogICAgICAibWV0YWRhdGEiIDogewogICAgICAgICJtb2RlbCIgOiAic2xpbSIKICAgICAgfQogICAgfQogIH0KfQ==".to_string(),
                    signature: None
                }
            ]
        })
    }
}

#[async_trait]
impl Services for ExampleImpl {
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
    build_managed(Box::new(ExampleImpl))
}
