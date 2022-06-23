pub mod auth;
pub mod session;

use okapi::openapi3::{Info, OpenApi};
use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, settings::OpenApiSettings};
use uuid::Uuid;

use crate::api::auth::{
    PayloadAuthenticate, PayloadInvalidate, PayloadRefresh, PayloadSignout, PayloadValidate,
    ResponseAuthenticate, ResponseRefresh,
};
use crate::api::session::{PayloadJoinServer, QueryHasJoined};
use crate::structs::session::{PlayerProperty, Profile};
use crate::structs::yggdrasil::{AuthenticationProfile, User};
use crate::traits::session::Session;
use crate::traits::{auth::Auth, YggoxideImpl};
use crate::Result;

pub struct ExampleImpl;

#[async_trait]
impl Auth for ExampleImpl {
    /// Authenticate a user with the service
    async fn authenticate(&self, data: PayloadAuthenticate) -> Result<ResponseAuthenticate> {
        let id = Uuid::new_v4().to_string();

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
                id: id.clone(),
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
                id: Uuid::new_v4().to_string(),
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
            id: "9fa80739429c46898f47eb0dfc8e5259".to_string(),
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
    async fn get_profile(&self, player_uuid: String) -> Result<Profile> {
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

impl YggoxideImpl for ExampleImpl {}

pub fn build() -> Rocket<Build> {
    let mut rocket = rocket::build();
    let settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "/" => (vec![], custom_openapi_spec()),
        "" => crate::api::auth::routes(),
        "/session" => crate::api::session::routes(),
    };

    let ygg: Box<dyn YggoxideImpl> = Box::new(ExampleImpl);

    rocket
        .manage(ygg)
        .mount("/authserver", auth::routes().0)
        .mount(
            "/swagger/",
            rocket_okapi::swagger_ui::make_swagger_ui(&rocket_okapi::swagger_ui::SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

fn custom_openapi_spec() -> OpenApi {
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            description: Some("It is important to note that all the \"Yggsdrasil Auth Server\" routes are also available with the prefix `/authserver`.".to_string()),
            ..Default::default()
        },
        ..Default::default()
    }
}
