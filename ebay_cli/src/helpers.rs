pub use ebay::client::Method;
use ebay::{auth::Credential, client::EbayClient};
use serde::Serialize;
use serde_json;
use std::io::stdout;

struct Env {
  pub host: String,
  pub credential: Credential,
  pub code: String,
  pub ru_name: String,
  pub refresh_token: String,
  pub access_token: String,
  pub trading_api_token: String,
}

fn get_env() -> Env {
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
    trading_api_token: var("TRADING_API_TOKEN").unwrap(),
  }
}

pub fn get_client() -> EbayClient {
  let env = get_env();
  EbayClient::new(
    &env.credential.client_id,
    &env.credential.client_secret,
    &env.refresh_token,
    &env.trading_api_token,
  )
  .finalize()
}

pub fn dump_json<T: Serialize>(v: T) {
  serde_json::to_writer_pretty(stdout(), &v).unwrap()
}
