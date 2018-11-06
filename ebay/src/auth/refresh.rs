//! Using a refresh token to update a User access token
//! [Doc](https://developer.ebay.com/api-docs/static/oauth-refresh-token-request.html)

use super::Credential;
use reqwest::Client;
use result::EbayResult;
use utils::read_ebay_response;

#[derive(Debug)]
pub struct Refresh<'a> {
  pub credential: &'a Credential,
  pub scopes: &'a [&'a str],
}

#[derive(Debug, Deserialize)]
pub struct RefreshResponse {
  pub access_token: String,
  pub expires_in: i64,
  pub token_type: String,
}

impl<'a> Refresh<'a> {
  pub fn refresh(&self, client: &Client, refresh_token: &str) -> EbayResult<RefreshResponse> {
    let url = "https://api.ebay.com/identity/v1/oauth2/token";

    #[derive(Debug, Serialize)]
    struct Form<'a> {
      grant_type: &'a str,
      refresh_token: &'a str,
      scope: String,
    }

    let mut resp = client
      .post(url)
      .basic_auth(
        &self.credential.client_id as &str,
        Some(&self.credential.client_secret as &str),
      ).form(&Form {
        grant_type: "refresh_token",
        refresh_token,
        scope: self.scopes.join(" "),
      }).send()?;

    check_resp!(resp);

    let resp = read_ebay_response(&mut resp)?;

    Ok(resp)
  }
}
