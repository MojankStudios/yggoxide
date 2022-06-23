use okapi::openapi3::OpenApi;
use rocket::Route;
use rocket_okapi::openapi_get_routes_spec;

pub mod authenticate;
pub mod index;
pub mod invalidate;
pub mod refresh;
pub mod signout;
pub mod validate;

pub use authenticate::*;
pub use index::*;
pub use invalidate::*;
pub use refresh::*;
pub use signout::*;
pub use validate::*;

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
