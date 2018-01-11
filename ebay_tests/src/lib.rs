extern crate dotenv;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;
extern crate ebay;

use reqwest::Client;
use ebay::auth::Credential;

lazy_static! {
  pub static ref HTTP_CLIENT: Client = {
    Client::new()
  };
}

mod auth;

pub struct Env {
  pub host: String,
  pub credential: Credential,
  pub code: String,
  pub ru_name: String,
}

pub fn get_env() -> Env {
  use std::env::var;
  ::dotenv::dotenv().unwrap();
  Env {
    host: var("HOST").unwrap(),
    credential: Credential {
      client_id: var("CLIENT_ID").unwrap(),
      client_secret: var("CLIENT_SECRET").unwrap(),
    },
    code: var("CODE").unwrap(),
    ru_name: var("RU_NAME").unwrap(),
  }
}