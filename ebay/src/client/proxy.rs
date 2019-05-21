use super::EbayClient;
use crate::result::EbayResult;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

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
      EbayHttpMethod::Get => Method::Get,
      EbayHttpMethod::Post => Method::Post,
      EbayHttpMethod::Put => Method::Put,
      EbayHttpMethod::Delete => Method::Delete,
    };
    let mut b = self.request(method, &req.path)?;

    if let Some(ref query) = req.query {
      b.query(query);
    }

    if let Some(ref headers) = req.headers {
      use reqwest::header::Headers;
      let mut add = Headers::new();
      for &(ref k, ref v) in headers {
        add.set_raw(k.to_string(), v.to_string());
      }
      b.headers(add);
    }

    let mut res = if let Some(ref body) = req.body {
      b.json(body).send()?
    } else {
      b.send()?
    };
    Ok(EbayResponse {
      headers: res
        .headers()
        .iter()
        .map(|view| (view.name().to_string(), view.value_string()))
        .collect(),
      status_code: res.status().as_u16(),
      body: res.json().ok(),
    })
  }
}
