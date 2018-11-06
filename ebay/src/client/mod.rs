use auth::Credential;
pub use reqwest::Method;
use reqwest::{Client, RequestBuilder, Response};
use result::EbayResult;
use serde::Deserialize;
use std::sync::RwLock;
use std::time::{Duration, Instant};

pub const ALL_SCOPES: [&'static str; 10] = [
  "https://api.ebay.com/oauth/api_scope",
  "https://api.ebay.com/oauth/api_scope/sell.marketing.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.marketing",
  "https://api.ebay.com/oauth/api_scope/sell.inventory.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.inventory",
  "https://api.ebay.com/oauth/api_scope/sell.account.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.account",
  "https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.fulfillment",
  "https://api.ebay.com/oauth/api_scope/sell.analytics.readonly",
];

struct AccessToken {
  token: String,
  expires: Instant,
}

pub struct EbayClient {
  http: Client,
  credential: Credential,
  refresh_token: String,
  trading_api_token: String,
  access_token: RwLock<Option<AccessToken>>,
  scopes: Vec<&'static str>,
}

pub struct EbayClientBuilder {
  inner: EbayClient,
}

impl EbayClientBuilder {
  pub fn scopes(&mut self, scopes: &[&'static str]) -> &mut Self {
    self.inner.scopes = scopes.iter().cloned().collect();
    self
  }

  pub fn finalize(&mut self) -> EbayClient {
    let reset = EbayClient {
      credential: self.inner.credential.clone(),
      http: self.inner.http.clone(),
      refresh_token: self.inner.refresh_token.clone(),
      trading_api_token: self.inner.trading_api_token.clone(),
      access_token: RwLock::new(None),
      scopes: ALL_SCOPES.iter().cloned().collect(),
    };
    ::std::mem::replace(&mut self.inner, reset)
  }
}

impl EbayClient {
  pub fn new(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
    trading_api_token: &str,
  ) -> EbayClientBuilder {
    Self::with_http_client(
      client_id,
      client_secret,
      refresh_token,
      trading_api_token,
      Client::new(),
    )
  }

  pub fn with_http_client(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
    trading_api_token: &str,
    http: Client,
  ) -> EbayClientBuilder {
    let credential = Credential {
      client_id: client_id.to_owned(),
      client_secret: client_secret.to_owned(),
    };
    let inner = EbayClient {
      credential,
      http,
      refresh_token: refresh_token.to_owned(),
      trading_api_token: trading_api_token.to_owned(),
      access_token: RwLock::new(None),
      scopes: ALL_SCOPES.iter().cloned().collect(),
    };

    EbayClientBuilder { inner }
  }

  fn refresh_access_token(&self) -> EbayResult<String> {
    use auth::Refresh;
    let r = Refresh {
      credential: &self.credential,
      scopes: &self.scopes,
    };
    let res = r.refresh(&self.http, &self.refresh_token)?;
    let expires = Instant::now() + Duration::from_secs(res.expires_in as u64);
    let value = res.access_token.clone();
    let token = AccessToken {
      token: res.access_token,
      expires,
    };

    let mut lock = self.access_token.write().unwrap();
    ::std::mem::replace(&mut lock as &mut Option<AccessToken>, Some(token));

    Ok(value)
  }

  fn get_access_token(&self) -> EbayResult<String> {
    let token = {
      let lock = self.access_token.read().unwrap();
      let token: &Option<AccessToken> = &lock;
      match *token {
        Some(ref t) => {
          if t.expires > Instant::now() + Duration::from_secs(30) {
            Some(t.token.clone())
          } else {
            None
          }
        }
        None => None,
      }
    };
    match token {
      Some(token) => Ok(token),
      None => self.refresh_access_token(),
    }
  }

  pub fn request(&self, method: Method, path: &str) -> EbayResult<RequestBuilder> {
    use reqwest::header::{Authorization, Bearer};
    let mut b = self
      .http
      .request(method, &format!("https://api.ebay.com{}", path));
    b.header(Authorization(Bearer {
      token: self.get_access_token()?,
    }));
    Ok(b)
  }
}

pub trait EbayResponse {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> EbayResult<T>;
}

impl EbayResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> EbayResult<T> {
    ::utils::read_ebay_response(self)
  }
}
