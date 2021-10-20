use reqwest::StatusCode;
use crate::trading::types::Error as TradingApiError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EbayError {
  #[error("request error: path = '{path}', status = '{status}', body = '{body}'")]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[error("deserialize body error: msg = '{msg}', body = '{body}'")]
  Deserialize { msg: String, body: String },

  #[error("url error: {0}")]
  Url(#[from] url::ParseError),

  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("{0}")]
  Msg(String),

  #[error("xml error: {0}")]
  Xml(#[from] xmltree::Error),

  #[error("utf8 error: {0}")]
  Utf8(#[from] std::str::Utf8Error),

  #[error("trading api response error: {0:?}")]
  TradingApiResponseError(Vec<TradingApiError>),

  #[error("invalid header value: {0:?}")]
  InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

  #[error("invalid header name: {0:?}")]
  InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),
}

impl EbayError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      EbayError::Request { status, .. } => {
        let code = status.as_u16();
        // 429 Too Many Requests
        code == 429 || code == 500 || code == 503
      }
      _ => false,
    }
  }
}

pub type EbayResult<T> = ::std::result::Result<T, EbayError>;