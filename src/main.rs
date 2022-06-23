#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod api;
pub mod r#impl;
pub mod structs;
pub mod traits;

pub use structs::error::{Error, Result};

#[launch]
fn rocket() -> _ {
    api::build()
}
