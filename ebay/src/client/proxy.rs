use super::EbayClient;
use crate::result::EbayResult;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EbayHttpMethod {
  Get,
  Post,
  Put,
  Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EbayRequest {
  pub method: EbayHttpMethod,
  pub path: String,
  pub headers: Option<Vec<(String, String)>>,
  pub query: Option<Vec<(String, String)>>,
  pub body: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct EbayResponse {
  pub headers: HashMap<String, String>,
  pub status_code: u16,
  pub body: Option<Value>,
}

impl EbayClient {
  pub fn proxy_request(&self, req: &EbayRequest) -> EbayResult<EbayResponse> {
    let method = match req.method {
      EbayHttpMethod::Get => Method::GET,
      EbayHttpMethod::Post => Method::POST,
      EbayHttpMethod::Put => Method::PUT,
      EbayHttpMethod::Delete => Method::DELETE,
    };
    let mut b = self.request(method, &req.path)?;

    if let Some(ref query) = req.query {
      b = b.query(query);
    }

    if let Some(ref headers) = req.headers {
      use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
      let mut add = HeaderMap::new();
      for &(ref k, ref v) in headers {
        add.insert(HeaderName::from_bytes(k.as_bytes())?, HeaderValue::from_str(v)?);
      }
      b = b.headers(add);
    }

    let res = if let Some(ref body) = req.body {
      b.json(body).send()?
    } else {
      b.send()?
    };
    Ok(EbayResponse {
      headers: res
        .headers()
        .iter()
        .map(|(name, value)| {
          (
            name.to_string(),
            value.to_str().map(ToOwned::to_owned).unwrap_or_default(),
          )
        })
        .collect(),
      status_code: res.status().as_u16(),
      body: res.json().ok(),
    })
  }
}
