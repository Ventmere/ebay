use reqwest::Response;
use result::*;
use serde::Deserialize;
use serde_json;

#[doc(hidden)]
#[macro_export]
macro_rules! check_resp {
  ($resp:expr) => {{
    use result::EbayError;
    if !$resp.status().is_success() {
      let body = $resp.text()?;
      return Err(EbayError::Request {
        path: $resp.url().to_string(),
        status: $resp.status(),
        body,
      });
    }
  }};
}

pub fn read_ebay_response<T: for<'de> Deserialize<'de>>(resp: &mut Response) -> EbayResult<T> {
  let body = resp.text()?;

  if !resp.status().is_success() {
    return Err(EbayError::Request {
      path: resp.url().to_string(),
      status: resp.status(),
      body,
    });
  }

  match serde_json::from_str(&body) {
    Ok(v) => Ok(v),
    Err(err) => {
      return Err(EbayError::Deserialize {
        msg: err.to_string(),
        body,
      })
    }
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! uppercase_str_enum {
  (pub enum $name:ident { $($v:ident,)+ }) => {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum $name {
      $(
        $v
      ),*
    }
  };
}
