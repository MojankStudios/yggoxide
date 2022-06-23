pub mod auth;
pub mod session;

use okapi::openapi3::{Info, OpenApi};
use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, settings::OpenApiSettings};

pub fn build() -> Rocket<Build> {
    let mut rocket = rocket::build();
    let settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), settings,
        "/" => (vec![], custom_openapi_spec()),
        "" => auth::routes(),
        "/session" => session::routes(),
    };

    rocket.mount("/authserver", auth::routes().0).mount(
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
