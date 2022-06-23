use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket_okapi::openapi_get_routes_spec;

mod has_joined;
mod join;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![join::join, has_joined::has_joined]
}
