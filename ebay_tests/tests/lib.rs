extern crate dotenv;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;
extern crate ebay;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use ebay::auth::Credential;
use ebay::client::EbayClient;
use reqwest::Client;

lazy_static! {
  pub static ref HTTP_CLIENT: Client = { Client::new() };
}

mod auth;
mod sell;

pub struct Env {
  pub host: String,
  pub credential: Credential,
  pub code: String,
  pub ru_name: String,
  pub refresh_token: String,
  pub access_token: String,
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
    refresh_token: var("REFRESH_TOKEN").unwrap(),
    access_token: var("ACCESS_TOKEN").unwrap(),
  }
}

pub fn get_client() -> EbayClient {
  let env = get_env();
  EbayClient::with_http_client(
    &env.credential.client_id,
    &env.credential.client_secret,
    &env.refresh_token,
    HTTP_CLIENT.clone(),
  ).finalize()
}

#[cfg(target_os = "macos")]
pub fn os_open_url(url: &str) {
  use std::process::Command;

  let mut child = Command::new("open").arg(url).spawn().unwrap();
  child.wait().unwrap();
}

#[cfg(not(target_os = "macos"))]
pub fn os_open_url(_url: &str) {
  unimplemented!()
}
