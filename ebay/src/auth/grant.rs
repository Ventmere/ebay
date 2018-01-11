//! Getting third-party permissions
//! [Doc](https://developer.ebay.com/api-docs/static/oauth-permissions-grant-request.html)

use super::Credential;
use result::EbayResult;

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
pub struct Grant<'a> {
  pub credential: &'a Credential,
  /// eBay uses RuName as redirect_url
  /// [Doc](https://developer.ebay.com/api-docs/static/oauth-redirect-uri.html)
  pub ru_name: &'a str,
  pub scopes: &'a [&'a str],
}

impl<'a> Grant<'a> {
  pub fn build_grant_url<S: ToString>(&self, state: S) -> EbayResult<GrantUrl> {
    use url::Url;

    let base = "https://auth.ebay.com/oauth2/authorize";
    let u = Url::parse_with_params(
      base,
      &[
        ("client_id", self.credential.client_id.clone()),
        ("redirect_uri", self.ru_name.to_string()),
        ("response_type", "code".to_string()),
        ("state", state.to_string()),
        ("scope", self.scopes.join(" ")),
      ],
    )?;

    Ok(GrantUrl {
      url: u.to_string(),
      state: state.to_string(),
    })
  }
}
