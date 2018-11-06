extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate xmltree;

pub mod result;
#[macro_use]
mod utils;
pub mod auth;
pub mod client;
pub mod sell;
// pub mod trading;
pub mod types;
