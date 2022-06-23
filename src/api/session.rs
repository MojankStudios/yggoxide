use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket_okapi::openapi_get_routes_spec;

pub mod has_joined;
pub mod join;
pub mod profile;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![join::join, has_joined::has_joined, profile::fetch_profile]
}
