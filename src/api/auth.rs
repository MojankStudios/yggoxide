use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket_okapi::openapi_get_routes_spec;

mod authenticate;
mod index;
mod invalidate;
mod refresh;
mod signout;
mod validate;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        index::index,
        authenticate::authenticate,
        refresh::refresh,
        validate::validate,
        signout::signout,
        invalidate::invalidate,
    ]
}
