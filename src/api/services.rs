use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket_okapi::openapi_get_routes_spec;

pub mod attributes;
pub mod certificates;

pub use attributes::*;
pub use certificates::*;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        attributes::fetch_attributes,
        certificates::fetch_certificates
    ]
}
