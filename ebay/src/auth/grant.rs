//! Getting third-party permissions
//! [Doc](https://developer.ebay.com/api-docs/static/oauth-permissions-grant-request.html)

use result::EbayResult;
use super::Credential;

#[derive(Debug)]
pub struct GrantUrl {
  url: String,
  state: String,
}

impl ToString for GrantUrl {
  fn to_string(&self) -> String {
    self.url.clone()
  }
}

#[derive(Debug)]
pub struct Grant {
  pub host: &'static str,
  pub credential: Credential,
  /// eBay uses RuName as redirect_url
  /// [Doc](https://developer.ebay.com/api-docs/static/oauth-redirect-uri.html)
  pub ru_name: String,
  pub scopes: Vec<String>,
}

impl Grant {
  pub fn build_grant_url<S: ToString>(&self, state: S) -> EbayResult<GrantUrl> {
    use url::Url;

    let base = format!("https://{}/authorize", self.host);
    let u = Url::parse_with_params(&base, &[
      ("client_id", self.credential.client_id.clone()),
      ("redirect_uri", self.ru_name.clone()),
      ("response_type", "code".to_string()),
      ("state", state.to_string()),
      ("scope", self.scopes.join(" ")),
    ])?;

    Ok(GrantUrl {
      url: u.to_string(),
      state: state.to_string(),
    })
  }
}
