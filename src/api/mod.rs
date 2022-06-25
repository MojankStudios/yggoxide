pub mod auth;
pub mod services;
pub mod session;

use okapi::openapi3::{Info, OpenApi};
use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, settings::OpenApiSettings};

use crate::traits::YggoxideImpl;

pub fn build_managed(ygg: Box<dyn YggoxideImpl>) -> Rocket<Build> {
    let mut rocket = rocket::build();
    let settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "/" => (vec![], custom_openapi_spec()),
        "" => crate::api::auth::routes(),
        "/session" => crate::api::session::routes(),
        "/services" => crate::api::services::routes(),
    };

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
