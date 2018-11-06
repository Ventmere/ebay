//! Exchanging the authorization code for a User access token
//! [Doc](https://developer.ebay.com/api-docs/static/oauth-auth-code-grant-request.html)

use super::Credential;
use reqwest::Client;
use result::EbayResult;
use utils::read_ebay_response;

#[derive(Debug)]
pub struct Exchange<'a> {
  pub credential: &'a Credential,
  /// eBay uses RuName as redirect_url
  /// [Doc](https://developer.ebay.com/api-docs/static/oauth-redirect-uri.html)
  pub ru_name: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
  pub access_token: String,
  pub expires_in: i64,
  pub refresh_token: String,
  pub refresh_token_expires_in: i64,
  pub token_type: String,
}

impl<'a> Exchange<'a> {
  pub fn exchange(&self, client: &Client, code: &str) -> EbayResult<ExchangeResponse> {
    let url = "https://api.ebay.com/identity/v1/oauth2/token";

    #[derive(Debug, Serialize)]
    struct Form<'a> {
      grant_type: &'a str,
      code: &'a str,
      redirect_uri: &'a str,
    }

    let mut resp = client
      .post(url)
      .basic_auth(
        &self.credential.client_id as &str,
        Some(&self.credential.client_secret as &str),
      ).form(&Form {
        grant_type: "authorization_code",
        code: code.as_ref(),
        redirect_uri: &self.ru_name,
      }).send()?;

    check_resp!(resp);

    let resp = read_ebay_response(&mut resp)?;

    Ok(resp)
  }
}
