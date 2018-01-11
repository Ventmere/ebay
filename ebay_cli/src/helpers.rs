use ebay::{auth::Credential, client::EbayClient};

struct Env {
  pub host: String,
  pub credential: Credential,
  pub code: String,
  pub ru_name: String,
  pub refresh_token: String,
  pub access_token: String,
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
  }
}

pub fn get_client() -> EbayClient {
  let env = get_env();
  EbayClient::new(
    &env.credential.client_id,
    &env.credential.client_secret,
    &env.refresh_token,
  ).finalize()
}
