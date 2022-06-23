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

use rocket::State;
pub use structs::error::{Error, Result};

pub type Ygg = State<Box<dyn traits::YggoxideImpl>>;

#[launch]
fn rocket() -> _ {
    api::build()
}
