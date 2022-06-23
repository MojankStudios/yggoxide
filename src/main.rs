#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate serde;

pub mod api;
pub mod r#impl;
pub mod structs;

#[launch]
fn rocket() -> _ {
    api::build()
}
